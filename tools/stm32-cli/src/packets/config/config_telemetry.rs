use nom::{number::streaming::le_u8, IResult};
use serde::Deserialize;

use super::PHYsecPayload;

#[repr(C, packed)]
#[derive(Debug, Deserialize, Clone)]
pub struct TelemetryConfigPacket {
    pub enabled: bool,
    pub logging_enabled: bool,
    pub keygen_info_enabled: bool,
    #[serde(skip)]
    pub rfu: u8,
}

impl TelemetryConfigPacket {
    pub fn new(enabled: bool, logging_enabled: bool, keygen_info_enabled: bool, rfu: u8) -> Self {
        Self {
            enabled,
            logging_enabled,
            keygen_info_enabled,
            rfu,
        }
    }

    pub fn from_bits(bits: u8) -> Self {
        Self {
            enabled: bits & 0b00000001 != 0,
            logging_enabled: bits & 0b00000010 != 0,
            keygen_info_enabled: bits & 0b00000100 != 0,
            rfu: (bits & 0b11111000) >> 3,
        }
    }

    pub fn to_bits(&self) -> u8 {
        (self.enabled as u8)
            | ((self.logging_enabled as u8) << 1)
            | ((self.keygen_info_enabled as u8) << 2)
            | (self.rfu << 3)
    }
}

impl PHYsecPayload for TelemetryConfigPacket {
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.to_bits()]
    }

    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, bits) = le_u8(input)?;
        Ok((input, TelemetryConfigPacket::from_bits(bits)))
    }
}

impl Default for TelemetryConfigPacket {
    fn default() -> Self {
        Self {
            enabled: true,
            logging_enabled: true,
            keygen_info_enabled: true,
            rfu: 0,
        }
    }
}
