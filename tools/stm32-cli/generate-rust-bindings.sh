#!/bin/bash

gen_libphysec_bindings() {
  bindgen ../../libphysec/libphysec.h --ignore-methods -o src/physec_bindings/libphysec.rs

  sed -si 's/quant_type_t_QUANT/QUANT/g' src/physec_bindings/libphysec.rs
  sed -si 's/csi_type_t_CSI/CSI/g' src/physec_bindings/libphysec.rs
  sed -si 's/preprocess_type_t_PREPROCESS/PREPROCESS/g' src/physec_bindings/libphysec.rs
  sed -si 's/recon_type_t_RECON/RECON/g' src/physec_bindings/libphysec.rs
  sed -si 's/physec_packet_type_t_PHYSEC/PHYSEC/g' src/physec_bindings/libphysec.rs

  sed -si 's/__IncompleteArrayField<u8>/[u8; 0]/g' src/physec_bindings/libphysec.rs

  sed -i "1i\#![allow(warnings)]" src/physec_bindings/libphysec.rs
}

gen_physec_serial_bindings() {
  bindgen --ignore-methods ../../STM32PlatformCode/Firmware/SubGHz_Phy/App/physec_serial.h -o src/physec_bindings/physec_serial.rs

  sed -si 's/__IncompleteArrayField<u8>/[u8; 0]/g' src/physec_bindings/physec_serial.rs
  sed -si 's/__IncompleteArrayField<i16>/[i16; 0]/g' src/physec_bindings/physec_serial.rs
  sed -si 's/physec_physical_layer_config__bindgen_ty_1/phy_layer_radio_config/g' src/physec_bindings/physec_serial.rs

  sed -i '1i\#![allow(warnings)]' src/physec_bindings/physec_serial.rs
}

gen_libphysec_bindings
gen_physec_serial_bindings
