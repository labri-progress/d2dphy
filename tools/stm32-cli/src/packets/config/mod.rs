pub mod config_csis;
pub mod config_keygen;
pub mod config_radio;
pub mod config_telemetry;

use crate::physec_bindings::physec_serial::*;
use config_keygen::KeyGenConfigPacket;
use nom::{
    number::streaming::{le_u32, le_u8},
    IResult,
};
use std::fmt::Debug;

use super::PHYsecPayload;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PHYsecConfigType {
    /// packet type ID of `KeyGenConfigPacket`
    PHYsecConfigKeyGen = PHYSEC_CONFIG_KEYGEN,
    /// packet type ID of `TelemetryConfigPacket`
    PHYsecConfigTelemetry = PHYSEC_CONFIG_TELEMETRY,
    /// packet type ID of `RadioConfigPacket`
    PHYsecConfigRadio = PHYSEC_CONFIG_RADIO,
    /// packet type ID of `CSIPacket`
    PHYsecConfigLoadCSIs = PHYSEC_CONFIG_LOAD_CSIS,
    /// packet type ID for starting config
    PHYsecConfigDone = PHYSEC_CONFIG_DONE,
    /// packet type ID for finishing config
    PHYsecConfigStart = PHYSEC_CONFIG_START,
}

impl TryFrom<u8> for PHYsecConfigType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            PHYSEC_CONFIG_KEYGEN => PHYsecConfigType::PHYsecConfigKeyGen,
            PHYSEC_CONFIG_TELEMETRY => PHYsecConfigType::PHYsecConfigTelemetry,
            PHYSEC_CONFIG_RADIO => PHYsecConfigType::PHYsecConfigRadio,
            PHYSEC_CONFIG_LOAD_CSIS => PHYsecConfigType::PHYsecConfigLoadCSIs,
            PHYSEC_CONFIG_DONE => PHYsecConfigType::PHYsecConfigDone,
            PHYSEC_CONFIG_START => PHYsecConfigType::PHYsecConfigStart,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct PHYsecConfigPacket {
    pub magic: u32,
    pub config_type: PHYsecConfigType,
    pub payload: Option<Box<dyn PHYsecPayload>>,
}

impl PHYsecConfigPacket {
    pub fn new(config_type: PHYsecConfigType, payload: Option<Box<dyn PHYsecPayload>>) -> Self {
        Self {
            magic: UART_PHYSEC_CONFIG_MAGIC,
            config_type,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let payload_bytes = match &self.payload {
            Some(payload) => payload.to_bytes(),
            None => Vec::new(),
        };
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.magic.to_le_bytes());
        bytes.push(self.config_type as u8);
        bytes.extend_from_slice(&payload_bytes);
        bytes
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, magic) = le_u32(input)?;
        let (input, config_type_byte) = le_u8(input)?;
        let config_type = PHYsecConfigType::try_from(config_type_byte).map_err(|e| {
            nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
        })?;

        let (input, payload) = match config_type {
            PHYsecConfigType::PHYsecConfigKeyGen => {
                let (input, payload) = KeyGenConfigPacket::from_bytes(input)?;
                (input, Some(Box::new(payload) as Box<dyn PHYsecPayload>))
            }
            _ => (input, None), // Handle other types if needed (for now we only receive keygen
                                // from stm32)
        };

        Ok((
            input,
            PHYsecConfigPacket {
                magic,
                config_type,
                payload,
            },
        ))
    }
}
