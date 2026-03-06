use nom::{number::streaming::le_u8, IResult};
use serde::Deserialize;

use crate::{packets::RADIO_CONFIG_UNION_SIZE, physec_bindings::physec_serial::*};

use super::PHYsecPayload;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum RadioModulation {
    LoRa = PHYSEC_PHY_MODULATION_LORA,
    FSK = PHYSEC_PHY_MODULATION_FSK,
}

impl TryFrom<usize> for RadioModulation {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(match v as u8 {
            PHYSEC_PHY_MODULATION_LORA => RadioModulation::LoRa,
            PHYSEC_PHY_MODULATION_FSK => RadioModulation::FSK,
            _ => return Err(()),
        })
    }
}

#[repr(u8)]
pub enum LoRaRadioBandwidth {
    BW125 = 0,
    BW250 = 1,
    BW500 = 2,
}

#[derive(Debug)]
pub struct RadioConfigPacket {
    pub modulation: RadioModulation,
    pub radio_config: Box<dyn PHYsecPayload>,
}

impl RadioConfigPacket {
    pub fn new(modulation: RadioModulation, radio_config: Box<dyn PHYsecPayload>) -> Self {
        Self {
            modulation,
            radio_config,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoRaRadioConfig {
    pub spreading_factor: u8,
    pub bandwidth: u8,
    pub tx_power: u8,
}

impl LoRaRadioConfig {
    pub fn new(spreading_factor: u8, bandwidth: u8, tx_power: u8) -> Self {
        Self {
            spreading_factor,
            bandwidth,
            tx_power,
        }
    }
}

impl Default for LoRaRadioConfig {
    fn default() -> Self {
        Self {
            spreading_factor: 7,
            bandwidth: LoRaRadioBandwidth::BW125 as u8,
            tx_power: 14,
        }
    }
}

impl PHYsecPayload for LoRaRadioConfig {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; *RADIO_CONFIG_UNION_SIZE];
        assert!(bytes.len() >= std::mem::size_of::<LoRaRadioConfig>());
        bytes[0] = self.spreading_factor;
        bytes[1] = self.bandwidth;
        bytes[2] = self.tx_power;
        bytes
    }

    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, spreading_factor) = le_u8(input)?;
        if spreading_factor < 6 || spreading_factor > 12 {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        let (input, bandwidth) = le_u8(input)?;
        if bandwidth > LoRaRadioBandwidth::BW500 as u8 {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        let (input, tx_power) = le_u8(input)?;
        if tx_power < 2 || tx_power > 17 {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        Ok((
            input,
            LoRaRadioConfig {
                spreading_factor,
                bandwidth,
                tx_power,
            },
        ))
    }
}

impl PHYsecPayload for RadioConfigPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.modulation as u8);
        bytes.extend_from_slice(&self.radio_config.to_bytes());
        bytes
    }

    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, modulation) = le_u8(input)?;
        let modulation = RadioModulation::try_from(modulation as usize).map_err(|e| {
            nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
        })?;
        let (input, radio_config) = match modulation {
            RadioModulation::LoRa => {
                let (input, radio_config) = LoRaRadioConfig::from_bytes(input)?;
                (input, Box::new(radio_config) as Box<dyn PHYsecPayload>)
            }
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )))
            }
        };
        Ok((
            input,
            RadioConfigPacket {
                modulation,
                radio_config,
            },
        ))
    }
}
