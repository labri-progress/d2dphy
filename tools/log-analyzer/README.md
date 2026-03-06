# PoC metrics
aims to extract metrics (KAR/KGR for example) from post mortem log analysis.

## More Precisely
The goal is, from two log files (at the moment, later on probably from CSI binary measures as well as energy measures) of a single / multiple key (multiple not implemented yet on the platform) generations using the PHYSEC_stm32 (and later on libphysec)  be able to extract meaningful informations

### What Exactly Do We Want ?
**For both whole experiments (multiple key generations trated as one single blob) and single key generation (the inner generations of the whole experiment), we wanna compute multiple metrics. More will be added later on**

**Also note that as I have more experience in GPU programming than embedded programming, I tend to qualify the STM32 boards as the `device` and the computer running the experiments as the `host`. So the `host` is never the board.**

#### For the whole experiment
- Time Date (information relative to when the experiment took time / its duration)
  - Start / End time on the `host` of all the key generations that took place during the experiment. For now it is always available as it is computed from the rust tool when receiving a UART packet, so it is only 'relatively' precise (most likely precise enough).
  - Total Elapsed Time (in seconds), the time it took to perform all the key generations on the `device`.
- Key Generations Parameters, the parameters of the simulation for both Alice (usually master) and Bob (usually slave)
  - is_master, a boolean telling if its the keygen's master. For more coherence let's always keep Alice the master !
  - csi_type, the method used for acquisition, derived from the generated bindgen (so new methods => new bindgen)
  - pre_process_type, the pre processing method used - if any, derived from the generated bindgen as well.
  - quant_type, the quantization method after the eventual pre-processing, also derived from bindgen.
  - recon_type, the information reconciliation method used, guess from where it's derived!!!!
  - probe_delay, the delay between two probes.
- Key Entropy:
  - bit entropy, the overall entropy of all the generated keys treated as a single blob (todo)
- All the single key generations (described below)

#### For a single key generation
- Time Data, same as for all the key generations but for the single key generation.
- Total number of received bits for both Alice and Bob
- Key Generation Rate (KGR) in bps
- Key Agreement Rate (KAR) and Hamming Distance for each step (quantization, post-processing, post-information reconciliation)
- Key Entropy:
  - bit entropy, the overall entropy of all the generated keys treated as a single blob (todo)


