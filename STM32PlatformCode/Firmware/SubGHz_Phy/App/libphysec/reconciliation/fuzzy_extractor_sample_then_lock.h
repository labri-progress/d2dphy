#pragma once

#include "../types.h"
#include <stdbool.h>
#include <stdint.h>

uint32_t dl_lock(uint8_t *key, uint32_t key_size, uint8_t *value,
                 uint32_t value_size, uint32_t security, uint8_t *nonce_buf,
                 uint32_t nonce_size, uint8_t *cipher_buf, physec_prng_t *prng);
bool dl_unlock(uint8_t *key, uint32_t key_size, uint8_t *cipher,
               uint32_t cipher_size, uint8_t *nonce, uint32_t nonce_size,
               uint32_t security, uint8_t *out_buf, uint8_t *temp_buffer,
               physec_prng_t *prng);

typedef struct fe_helpers_s {
  uint8_t **ciphers;
  uint8_t **nonces;
  uint8_t **masks;
} fe_helpers_t;

void fe_gen(uint8_t *w, uint32_t w_size, uint32_t l, uint32_t k, uint8_t *r_buf,
            uint32_t r_size, uint32_t security, uint32_t nonce_size,
            fe_helpers_t *helpers, uint8_t *w_i_buf, physec_prng_t *r_prng,
            physec_prng_t *m_prng, physec_prng_t *l_prng);

bool fe_rep(uint8_t *w_p, uint32_t w_p_size, uint32_t l, uint32_t s,
            uint8_t *r_buf, uint32_t r_size, uint32_t nonces_size,
            uint8_t *w_i_buf, fe_helpers_t *helpers, uint8_t *temp_buf,
            physec_prng_t *prng);
