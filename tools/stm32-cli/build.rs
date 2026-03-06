use regex::Regex;
use std::fs;
use std::path::PathBuf;

fn main() {
    generate_libphysec_bindings();
    generate_physec_serial_bindings();
    println!("cargo:rerun-if-changed=../../STM32PlatformCode/Firmware/SubGHz_Phy/App/libphysec/libphysec.h");
    println!("cargo:rerun-if-changed=../../STM32PlatformCode/Firmware/SubGHz_Phy/App/physec_serial.h");
}

fn generate_libphysec_bindings() {
    let bindings = bindgen::Builder::default()
        .header("../../STM32PlatformCode/Firmware/SubGHz_Phy/App/libphysec/libphysec.h")
        .ignore_methods()
        .generate()
        .expect("Unable to generate bindings for libphysec");

    let bindings_path = PathBuf::from("src/physec_bindings/libphysec.rs");
    let mut bindings_content = bindings.to_string();

    let re_quant = Regex::new(r"quant_type_t_QUANT").unwrap();
    bindings_content = re_quant.replace_all(&bindings_content, "QUANT").to_string();

    let re_csi = Regex::new(r"csi_type_t_CSI").unwrap();
    bindings_content = re_csi.replace_all(&bindings_content, "CSI").to_string();

    let re_preprocess = Regex::new(r"preprocess_type_t_PREPROCESS").unwrap();
    bindings_content = re_preprocess
        .replace_all(&bindings_content, "PREPROCESS")
        .to_string();

    let re_recon = Regex::new(r"recon_type_t_RECON").unwrap();
    bindings_content = re_recon.replace_all(&bindings_content, "RECON").to_string();

    let re_physec = Regex::new(r"physec_packet_type_t_PHYSEC").unwrap();
    bindings_content = re_physec
        .replace_all(&bindings_content, "PHYSEC")
        .to_string();

    let re_incomplete_array = Regex::new(r"__IncompleteArrayField<u8>").unwrap();
    bindings_content = re_incomplete_array
        .replace_all(&bindings_content, "[u8; 0]")
        .to_string();

    bindings_content = format!("#![allow(warnings)]\n{}", bindings_content);

    fs::write(bindings_path, bindings_content).expect("Couldn't write bindings");
}

fn generate_physec_serial_bindings() {
    let bindings = bindgen::Builder::default()
        .header("../../STM32PlatformCode/Firmware/SubGHz_Phy/App/physec_serial.h")
        .ignore_methods()
        .clang_arg("-DSTM32L072xx")
        .clang_arg("-I../../STM32PlatformCode/Firmware/Core/Inc/")
        .clang_arg("-I../../STM32PlatformCode/Drivers/STM32L0xx_HAL_Driver/Inc/")
        .clang_arg("-I../../STM32PlatformCode/Firmware/SubGHz_Phy/App")
        .clang_arg("-I../../STM32PlatformCode/Firmware/SubGHz_Phy/Target")
        .clang_arg("-I../../STM32PlatformCode/Firmware/Core/Inc")
        .clang_arg("-I../../STM32PlatformCode/Utilities/misc")
        .clang_arg("-I../../STM32PlatformCode/Utilities/timer")
        .clang_arg("-I../../STM32PlatformCode/Utilities/trace/adv_trace")
        .clang_arg("-I../../STM32PlatformCode/Utilities/lpm/tiny_lpm")
        .clang_arg("-I../../STM32PlatformCode/Utilities/sequencer")
        .clang_arg("-I../../STM32PlatformCode/Drivers/BSP/B-L072Z-LRWAN1")
        .clang_arg("-I../../STM32PlatformCode/Drivers/BSP/CMWX1ZZABZ_0xx")
        .clang_arg("-I../../STM32PlatformCode/Drivers/STM32L0xx_HAL_Driver/Inc")
        .clang_arg("-I../../STM32PlatformCode/Drivers/CMSIS/Device/ST/STM32L0xx/Include")
        .clang_arg("-I../../STM32PlatformCode/Drivers/CMSIS/Include")
        .clang_arg("-I../../STM32PlatformCode/Middlewares/Third_Party/SubGHz_Phy")
        .clang_arg("-I../../STM32PlatformCode/Middlewares/Third_Party/SubGHz_Phy/sx1276")
        .clang_arg("-I../../STM32PlatformCode/Firmware/SubGHz_Phy/App/libphysec")
        .generate()
        .expect("Unable to generate bindings for physec_serial");

    let bindings_path = PathBuf::from("src/physec_bindings/physec_serial.rs");
    let mut bindings_content = bindings.to_string();

    let re_incomplete_array_u8 = Regex::new(r"__IncompleteArrayField<u8>").unwrap();
    bindings_content = re_incomplete_array_u8
        .replace_all(&bindings_content, "[u8; 0]")
        .to_string();

    let re_incomplete_array_i16 = Regex::new(r"__IncompleteArrayField<i16>").unwrap();
    bindings_content = re_incomplete_array_i16
        .replace_all(&bindings_content, "[i16; 0]")
        .to_string();

    let re_phy_layer_radio_config =
        Regex::new(r"physec_physical_layer_config__bindgen_ty_1").unwrap();
    bindings_content = re_phy_layer_radio_config
        .replace_all(&bindings_content, "phy_layer_radio_config")
        .to_string();

    bindings_content = format!("#![allow(warnings)]\n{}", bindings_content);

    fs::write(bindings_path, bindings_content).expect("Couldn't write bindings");
}
