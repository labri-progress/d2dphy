use enum_iterator::{all, Sequence};

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
}
