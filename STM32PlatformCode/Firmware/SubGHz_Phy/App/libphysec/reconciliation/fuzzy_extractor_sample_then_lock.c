#include "fuzzy_extractor_sample_then_lock.h"
#include "reconciliation.h"
#include <stdbool.h>
#include <stdint.h>
uint32_t dl_lock(uint8_t *key, uint32_t key_size, uint8_t *value,
                 uint32_t value_size, uint32_t security, uint8_t *nonce_buf,
                 uint32_t nonce_size, uint8_t *cipher_buf,
                 physec_prng_t *prng) {

  // first get a nonce
  get_random_bytes(nonce_buf, nonce_size, prng);
  // then compute d = prf(key, nonce)
  prf(cipher_buf, value_size + security, key, key_size, nonce_buf, nonce_size,
      prng);
  // pad_val = val || 0s
  // finally compute c = d ^ pad_val
  byte_array_xor(cipher_buf, value_size + security, value, value_size);
  return value_size + security;
}

bool dl_unlock(uint8_t *key, uint32_t key_size, uint8_t *cipher,
               uint32_t cipher_size, uint8_t *nonce, uint32_t nonce_size,
               uint32_t security, uint8_t *out_buf, uint8_t *temp_buffer,
               physec_prng_t *prng) {
  prf(temp_buffer, cipher_size, key, key_size, nonce, nonce_size, prng);
  byte_array_xor(temp_buffer, cipher_size, cipher, cipher_size);
  bool worked = has_n_padding_zeros(temp_buffer, cipher_size, security);
  if (worked) {
    byte_array_copy_bytes(out_buf, temp_buffer, cipher_size - security);
  }
  return worked;
}

void fe_gen(uint8_t *w, uint32_t w_size, uint32_t l, uint32_t k, uint8_t *r_buf,
            uint32_t r_size, uint32_t security, uint32_t nonce_size,
            fe_helpers_t *helpers, uint8_t *w_i_buf, physec_prng_t *r_prng,
            physec_prng_t *m_prng, physec_prng_t *l_prng) {
  // first we choose r
  get_random_bytes(r_buf, r_size, r_prng);
  // then we do l locks from random subsamples of size k
  for (uint32_t l_idx = 0; l_idx < l; l_idx++) {
    // get a random mask
    get_random_sampling_mask(helpers->masks[l_idx], w_size, k, m_prng);
    // compute w_i
    byte_array_copy_bytes(w_i_buf, w, w_size);
    byte_array_and(w_i_buf, w_size, helpers->masks[l_idx], w_size);
    // and lock with it
    dl_lock(w_i_buf, w_size, r_buf, r_size, security, helpers->nonces[l_idx],
            nonce_size, helpers->ciphers[l_idx], l_prng);
  }
}

bool fe_rep(uint8_t *w_p, uint32_t w_p_size, uint32_t l, uint32_t s,
            uint8_t *r_buf, uint32_t r_size, uint32_t nonces_size,
            uint8_t *w_i_buf, fe_helpers_t *helpers, uint8_t *temp_buf,
            physec_prng_t *prng) {
  for (uint32_t l_idx = 0; l_idx < l; l_idx++) {
    byte_array_copy_bytes(w_i_buf, w_p, w_p_size);
    byte_array_and(w_i_buf, w_p_size, helpers->masks[l_idx], w_p_size);
    if (dl_unlock(w_i_buf, w_p_size, helpers->ciphers[l_idx], r_size + s,
                  helpers->nonces[l_idx], nonces_size, s, r_buf, temp_buf,
                  prng)) {
      return true;
    }
  }
  return false;
}
