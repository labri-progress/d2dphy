use console::style;
use nom::{bytes::streaming::take, number::streaming::le_u16, IResult};

use crate::packets::PHYsecPayload;

use super::PHYsecTelemetry;

#[derive(Debug)]
pub struct TelemetryLogging {
    size: u16,
    msg: String,
}

impl PHYsecPayload for TelemetryLogging {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(self.msg.as_bytes());
        bytes
    }

    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let total_size = input.len();
        let (input, size) = le_u16(input).map_err(|e| match e {
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
        let (input, msg) = take(size)(input).map_err(|e| match e {
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
        let datetime = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S-%.3f");
        Ok((
            input,
            TelemetryLogging {
                size,
                msg: format!("{} {}", datetime, String::from_utf8_lossy(msg).to_string()),
            },
        ))
    }
}

impl PHYsecTelemetry for TelemetryLogging {
    fn to_display(&self) -> String {
        let msg = self.to_log();
        style(msg).green().bright().to_string()
    }
    fn to_log(&self) -> String {
        let msg = if self.msg.len() > 2 {
            match &self.msg[self.msg.len() - 2..] {
                "\r\n" | "\n\r" => &self.msg[..self.msg.len() - 2],
                _ => &self.msg[..],
            }
        } else {
            &self.msg[..]
        };
        msg.to_string()
    }
}
