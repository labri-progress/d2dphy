use std::fs;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

fn main() {
    // build the c library
    let status = Command::new("make")
        .arg("-C")
        .arg("bindings/libsts")
        .arg("-f")
        .arg("makefile.bindings")
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Failed to build C library");
    }

    generate_physec_bindings();
    generate_sts_bindings();

    // tell cargo to link against libsts.a
    println!("cargo:rustc-link-search=bindings/libsts");
    println!("cargo:rustc-link-lib=static=sts");

    // tell Cargo to rerun this build script if any of these files change
    println!("cargo:rerun-if-changed=bindings/libphysec_bindgen_headers.h");
    println!("cargo:rerun-if-changed=bindings/libsts_bindgen_headers.h");
    println!("cargo:rerun-if-changed=bindings/physec_additional.rs");
    println!("cargo:rerun-if-changed=bindings/libsts/makefile.bindings");

    // source file used, in order to keep only what we use
    let c_files = vec![
        "bindings/libsts/src/approximateEntropy.c",
        //"bindings/libsts/src/assess.c",
        "bindings/libsts/src/blockFrequency.c",
        "bindings/libsts/src/cephes.c",
        "bindings/libsts/src/cusum.c",
        "bindings/libsts/src/dfft.c",
        "bindings/libsts/src/discreteFourierTransform.c",
        "bindings/libsts/src/externs.c",
        "bindings/libsts/src/frequency.c",
        //"bindings/libsts/src/generators.c",
        //"bindings/libsts/src/genutils.c",
        "bindings/libsts/src/linearComplexity.c",
        "bindings/libsts/src/longestRunOfOnes.c",
        //"bindings/libsts/src/matrix.c",
        "bindings/libsts/src/nonOverlappingTemplateMatchings.c",
        //"bindings/libsts/src/overlappingTemplateMatchings.c",
        //"bindings/libsts/src/randomExcursions.c",
        //"bindings/libsts/src/randomExcursionsVariant.c",
        //"bindings/libsts/src/rank.c",
        //"bindings/libsts/src/runs.c",
        //"bindings/libsts/src/serial.c",
        //"bindings/libsts/src/universal.c",
        //"bindings/libsts/src/utilities.c",
    ];
    for file in c_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    let include_headers = vec![
        "bindings/libsts/include/cephes.h",
        "bindings/libsts/include/config.h",
        "bindings/libsts/include/decls.h",
        "bindings/libsts/include/defs.h",
        "bindings/libsts/include/externs.h",
        //"bindings/libsts/include/generators.h",
        //"bindings/libsts/include/genutils.h",
        //"bindings/libsts/include/matrix.h",
        //"bindings/libsts/include/stat_fncs.h",
        //"bindings/libsts/include/utilities.h",
    ];
    for file in include_headers {
        println!("cargo:rerun-if-changed={}", file);
    }
}

fn generate_physec_bindings() {
    let bindings = bindgen::Builder::default()
        .header("bindings/libphysec_bindgen_headers.h")
        .allowlist_type("quant_type_t|csi_type_t|preprocess_type_t|recon_type_t")
        .rustified_enum("quant_type_t|csi_type_t|preprocess_type_t|recon_type_t")
        .generate()
        .expect("Unable to generate bindings for physec");

    let bindings_path = PathBuf::from("src/bindings/physec.rs");

    let bindings_content = bindings.to_string();

    let re = Regex::new(r"#\[derive\(([^)]*)\)\]").unwrap();
    let modified_content = re.replace_all(&bindings_content, |caps: &regex::Captures| {
        format!("#[derive({}, Sequence)]", &caps[1])
    });
    let mut headers = String::new();
    headers.push_str("#![allow(non_upper_case_globals)]\n");
    headers.push_str("#![allow(non_camel_case_types)]\n");
    headers.push_str("#![allow(non_snake_case)]\n");

    let additional_code = match fs::read_to_string("bindings/physec_additional.rs") {
        Ok(content) => content,
        Err(e) => panic!("Couldn't read additional bindings code: {}", e),
    };

    let final_content = format!("{}\n{}\n{}", headers, modified_content, additional_code);

    fs::write(&bindings_path, final_content).expect("Couldn't write bindings for physec");
}

fn generate_sts_bindings() {
    let bindings = bindgen::Builder::default()
        .header("bindings/libsts_bindgen_headers.h")
        .allowlist_type("approximate_entropy|block_frenquency|cusum|discrete_fourier_transform|frequency|linear_complexity|longest_run_of_ones|non_overlapping_template_matchings")
        .allowlist_function("ApproximateEntropy|BitSequence|BlockFrequency|CumulativeSums|DiscreteFourierTransform|Frequency|LinearComplexity|LongestRunOfOnes|NonOverlappingTemplateMatchings")
        .allowlist_var("epsilon")
        .generate()
        .expect("Unable to generate bindings for sts");

    let bindings_path = PathBuf::from("src/bindings/nist_sts.rs");

    let mut content = String::new();
    content.push_str("#![allow(non_upper_case_globals)]\n");
    content.push_str("#![allow(non_camel_case_types)]\n");
    content.push_str("#![allow(non_snake_case)]\n");
    content.push_str(&bindings.to_string());

    fs::write(&bindings_path, content).expect("Couldn't write bindings for sts");
}
