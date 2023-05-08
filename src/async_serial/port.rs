use std::{path::Path, time::Duration};

use serialport::{DataBits, Parity, SerialPort, StopBits};

use super::{Command, ProgrammingCommand};
use crate::error::{Error, Result};

const PROGRAMMING_UNLOCKING_SEQUENCE: &[u8] = &[0xCD, 0xEF, 0x89, 0xAB];
const PROGRAMMING_DELAY_BETWEEN_BYTES: Duration = Duration::from_millis(1);

pub struct Port {
    inner: Box<dyn SerialPort>,
}

impl Port {
    pub fn try_new(path: impl AsRef<Path>, baud_rate: u32, timeout: Duration) -> Result<Self> {
        let inner = serialport::new(path.as_ref().to_string_lossy(), baud_rate)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .parity(Parity::None)
            .timeout(timeout)
            .open()
            .map_err(|source| Error::AsyncSerialFailedToOpen {
                source,
                path: path.as_ref().into(),
            })?;

        Ok(Self { inner })
    }

    fn send_byte(&mut self, byte: u8) -> Result<()> {
        let buf = [byte];
        self.inner
            .write_all(&buf)
            .map_err(Error::AsyncSerialFailedToSend)?;
        Ok(())
    }

    fn drop_until(&mut self, byte: u8) -> Result<()> {
        let mut buf = [0; 1];
        loop {
            self.inner
                .read_exact(&mut buf)
                .map_err(Error::AsyncSerialFailedToReceive)?;
            if buf[0] == byte {
                return Ok(());
            }
        }
    }

    pub fn send_command(&mut self, command: &Command) -> Result<()> {
        self.send_byte(command.to_byte())
    }

    pub fn send_programming_command(&mut self, command: &ProgrammingCommand) -> Result<()> {
        let mut bytes = Vec::new();
        bytes.extend(PROGRAMMING_UNLOCKING_SEQUENCE);
        bytes.push(command.to_byte());
        if let Some(additional_data) = command.additional_data() {
            bytes.extend(additional_data);
        }

        for byte in bytes {
            self.send_byte(byte)?;
            std::thread::sleep(PROGRAMMING_DELAY_BETWEEN_BYTES);

            self.drop_until(byte)?;
        }

        Ok(())
    }

    pub fn receive(&mut self, buf: &mut impl AsMut<[u8]>) -> Result<()> {
        self.inner
            .read_exact(buf.as_mut())
            .map_err(Error::AsyncSerialFailedToReceive)?;
        Ok(())
    }
}
