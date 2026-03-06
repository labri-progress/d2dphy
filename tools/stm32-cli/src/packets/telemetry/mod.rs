use console::style;
use nom::{
    number::streaming::{le_u32, le_u8},
    IResult,
};
use telemetry_keygen_conf::TelemetryKeyGenConfig;
use telemetry_keygen_info::TelemetryKeyGenInfo;
use telemetry_logging::TelemetryLogging;

use super::{config::config_csis::CSIPacket, PHYsecPayload};

pub mod telemetry_csis;
pub mod telemetry_keygen_conf;
pub mod telemetry_keygen_info;
pub mod telemetry_logging;

use crate::physec_bindings::physec_serial::*;

pub trait PHYsecTelemetry: PHYsecPayload {
    /// Transform telemetry payload to a human-readable log message
    /// beautified with colors and styles
    fn to_display(&self) -> String;
    /// Transform telemetry payload to a log file entry
    fn to_log(&self) -> String;
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PHYsecTelemetryType {
    /// packet type ID of `TelemetryLogging`
    PHYsecTelemetryLogging = PHYSEC_TELEMETRY_LOGGING,
    /// packet type ID of `TelemetryKeyGenConfig`
    PHYsecTelemetryKeyGenConf = PHYSEC_TELEMETRY_KG_CONF,
    /// packet type ID of `TelemetryKeyGenInfo`
    PHYsecTelemetryKeyGenInfo = PHYSEC_TELEMETRY_KG_INFO,
    /// packet type ID of `CSIPacket`
    PHYsecTelemetryCSIs = PHYSEC_TELEMETRY_CSIS,
}

impl TryFrom<u8> for PHYsecTelemetryType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            PHYSEC_TELEMETRY_LOGGING => PHYsecTelemetryType::PHYsecTelemetryLogging,
            PHYSEC_TELEMETRY_KG_CONF => PHYsecTelemetryType::PHYsecTelemetryKeyGenConf,
            PHYSEC_TELEMETRY_KG_INFO => PHYsecTelemetryType::PHYsecTelemetryKeyGenInfo,
            PHYSEC_TELEMETRY_CSIS => PHYsecTelemetryType::PHYsecTelemetryCSIs,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct PHYsecTelemetryPacket {
    pub magic: u32,
    pub telemetry_type: PHYsecTelemetryType,
    pub payload: Option<Box<dyn PHYsecTelemetry>>,
}

impl PHYsecTelemetryPacket {
    pub fn new(
        telemetry_type: PHYsecTelemetryType,
        payload: Option<Box<dyn PHYsecTelemetry>>,
    ) -> Self {
        Self {
            magic: UART_PHYSEC_TELEMETRY_MAGIC,
            telemetry_type,
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
        bytes.push(self.telemetry_type as u8);
        bytes.extend_from_slice(&payload_bytes);
        bytes
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let total_size = input.len();
        let (input, magic) = le_u32(input).map_err(|e| match e {
            nom::Err::Failure(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Eof,
            })
            | nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Eof,
            }) => nom::Err::Incomplete(nom::Needed::new(total_size)),
            other => other,
        })?;
        if !magic == UART_PHYSEC_TELEMETRY_MAGIC {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        let (input, telemetry_type_byte) = le_u8(input).map_err(|e| match e {
            nom::Err::Failure(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Eof,
            })
            | nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Eof,
            }) => nom::Err::Incomplete(nom::Needed::new(total_size)),
            other => other,
        })?;
        let telemetry_type = PHYsecTelemetryType::try_from(telemetry_type_byte).map_err(|e| {
            nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;

        let (input, payload) = match telemetry_type {
            PHYsecTelemetryType::PHYsecTelemetryKeyGenConf => {
                let (input, payload) =
                    TelemetryKeyGenConfig::from_bytes(input).map_err(|e| match e {
                        nom::Err::Incomplete(needed) => {
                            let n = match needed {
                                nom::Needed::Size(n) => usize::from(n),
                                _ => TelemetryKeyGenConfig::mem_size(),
                            };
                            nom::Err::Incomplete(nom::Needed::new(
                                n + PHYsecTelemetryPacket::mem_size(),
                            ))
                        }
                        other => other,
                    })?;
                (input, Some(Box::new(payload) as Box<dyn PHYsecTelemetry>))
            }
            PHYsecTelemetryType::PHYsecTelemetryKeyGenInfo => {
                let (input, payload) =
                    TelemetryKeyGenInfo::from_bytes(input).map_err(|e| match e {
                        nom::Err::Incomplete(needed) => {
                            let n = match needed {
                                nom::Needed::Size(n) => usize::from(n),
                                _ => {
                                    return nom::Err::Incomplete(nom::Needed::Unknown);
                                }
                            };
                            nom::Err::Incomplete(nom::Needed::new(
                                n + PHYsecTelemetryPacket::mem_size(),
                            ))
                        }
                        other => other,
                    })?;
                (input, Some(Box::new(payload) as Box<dyn PHYsecTelemetry>))
            }
            PHYsecTelemetryType::PHYsecTelemetryLogging => {
                let (input, payload) =
                    TelemetryLogging::from_bytes(input).map_err(|e| match e {
                        nom::Err::Incomplete(needed) => {
                            let n = match needed {
                                nom::Needed::Size(n) => usize::from(n),
                                _ => {
                                    return nom::Err::Incomplete(nom::Needed::Unknown);
                                }
                            };
                            nom::Err::Incomplete(nom::Needed::new(
                                n + PHYsecTelemetryPacket::mem_size(),
                            ))
                        }
                        other => other,
                    })?;
                (input, Some(Box::new(payload) as Box<dyn PHYsecTelemetry>))
            }
            PHYsecTelemetryType::PHYsecTelemetryCSIs => {
                let (input, payload) = CSIPacket::from_bytes(input).map_err(|e| match e {
                    nom::Err::Incomplete(needed) => {
                        let n = match needed {
                            nom::Needed::Size(n) => usize::from(n),
                            _ => {
                                return nom::Err::Incomplete(nom::Needed::Unknown);
                            }
                        };
                        nom::Err::Incomplete(nom::Needed::new(
                            n + PHYsecTelemetryPacket::mem_size(),
                        ))
                    }
                    other => other,
                })?;

                (input, Some(Box::new(payload) as Box<dyn PHYsecTelemetry>))
            }
            // Implement parsing for specific payload types
            _ => (input, None), // Handle other types if needed
        };

        Ok((
            input,
            PHYsecTelemetryPacket {
                magic,
                telemetry_type,
                payload,
            },
        ))
    }

    pub fn to_display(&self) -> String {
        let tm_type = format!("{:?}", self.telemetry_type);
        match &self.payload {
            Some(payload) => format!("{}: {}", style(tm_type).bold(), payload.to_display()),
            None => format!("{}", style(tm_type).bold()),
        }
    }

    pub fn to_log(&self) -> String {
        let tm_type = format!("{:?}", self.telemetry_type);
        match &self.payload {
            Some(payload) => format!("{}: {}", tm_type, payload.to_log()),
            None => format!("{}", tm_type),
        }
    }

    pub fn mem_size() -> usize {
        std::mem::size_of::<u32>() + std::mem::size_of::<PHYsecTelemetryType>()
    }
}
