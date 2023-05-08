use super::*;

const SELF_CALIBRATION_STATUS_SIZE: usize = 1;

#[derive(Default)]
pub struct SelfCalibrationStatus {
    buf: [u8; SELF_CALIBRATION_STATUS_SIZE + 1],
}

impl SelfCalibrationStatus {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn status_byte(&self) -> u8 {
        self.buf[1]
    }

    pub fn is_out_of_range(&self) -> bool {
        (self.status_byte() & 0b00001000) != 0
    }

    pub fn is_timeout(&self) -> bool {
        (self.status_byte() & 0b00000100) != 0
    }

    pub fn counter(&self) -> u8 {
        self.status_byte() & 0b00000011
    }
}

impl PrefixedResponse for SelfCalibrationStatus {
    fn command() -> Command {
        Command::SelfCalibrationStatusRequest
    }

    fn prefix(&self) -> u8 {
        self.buf[0]
    }
}

impl AsMut<[u8]> for SelfCalibrationStatus {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}
