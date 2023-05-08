use std::fmt;

use super::*;

const SERIAL_NUMBER_LENGTH: usize = 6;

#[derive(Default)]
pub struct SerialNumber {
    buf: [u8; SERIAL_NUMBER_LENGTH + 1],
}

impl SerialNumber {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[1..]).unwrap()
    }
}

impl PrefixedResponse for SerialNumber {
    fn command() -> Command {
        Command::SerialNumber
    }

    fn prefix(&self) -> u8 {
        self.buf[0]
    }
}

impl AsMut<[u8]> for SerialNumber {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

impl fmt::Display for SerialNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
