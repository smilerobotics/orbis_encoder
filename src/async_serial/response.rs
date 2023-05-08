use super::Command;

pub trait PrefixedResponse {
    fn command() -> Command;
    fn prefix(&self) -> u8;

    fn is_valid_prefix(&self) -> bool {
        Self::command().to_byte() == self.prefix()
    }
}

mod position_and_status;
mod self_calibration_status;
mod serial_number;

pub use position_and_status::*;
pub use self_calibration_status::SelfCalibrationStatus;
pub use serial_number::SerialNumber;
