#!/bin/sh

export ENUM_LIST="quant_type_t|csi_type_t|preprocess_type_t|recon_type_t"

result=$(bindgen bindings/libphysec_bindgen_headers.h \
  --allowlist-type "$ENUM_LIST" \
  --rustified-enum "$ENUM_LIST")

echo "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use enum_iterator::{all, Sequence};
" >src/bindings/physec.rs

echo "$result" >>src/bindings/physec.rs

sed -i '/#\[derive/ s/\([^)]*\))/\1, Sequence)/' src/bindings/physec.rs

echo '
pub trait EnumValue {
    fn value(&self) -> u8;
}

impl EnumValue for csi_type_t {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl EnumValue for preprocess_type_t {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl EnumValue for quant_type_t {
    fn value(&self) -> u8 {
        *self as u8
    }
}

impl EnumValue for recon_type_t {
    fn value(&self) -> u8 {
        *self as u8
    }
}

pub trait RetrievableEnum<T> {
    fn retrieve(value: usize) -> Result<T, String>;
}

impl<T> RetrievableEnum<T> for T
where
    T: Sequence + Clone + PartialEq + EnumValue,
{
    fn retrieve(value: usize) -> Result<T, String> {
        all::<T>()
            .find(|variant| variant.value() as usize == value)
            .ok_or_else(|| format!("No enum variant found with value {value}"))
    }
}' >>src/bindings/physec.rs

cargo clean
