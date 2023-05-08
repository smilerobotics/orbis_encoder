#[derive(Clone, Copy, Debug)]
pub enum Command {
    PositionRequest = 0x31,
    ShortPositionRequest = 0x33,
    PositionRequestAndDetailedStatus = 0x64,
    PositionRequestAndTemperature = 0x74,
    SerialNumber = 0x76,
    SelfCalibrationStatusRequest = 0x69,
}

impl Command {
    pub(crate) fn to_byte(self) -> u8 {
        self as u8
    }
}
