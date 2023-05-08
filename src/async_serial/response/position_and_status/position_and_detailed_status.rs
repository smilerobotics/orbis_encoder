use super::*;
use crate::async_serial::{response::PrefixedResponse, Command};

pub struct PositionAndDetailedStatus {
    counter_type: CounterType,
    inner: PositionAndStatusInner,
}

impl PositionAndDetailedStatus {
    const DETAILED_STATUS_DATA_SIZE: usize = 1;
    const PREFIX_SIZE: usize = 1;

    pub fn new(counter_type: CounterType) -> Self {
        Self {
            counter_type,
            inner: PositionAndStatusInner::new(
                counter_type,
                Self::PREFIX_SIZE,
                Self::DETAILED_STATUS_DATA_SIZE,
            ),
        }
    }

    pub fn counter_type(&self) -> CounterType {
        self.counter_type
    }

    fn detailed_status(&self) -> u8 {
        self.inner.postfix().unwrap()[0]
    }

    pub fn is_signal_too_high(&self) -> bool {
        (self.detailed_status() & 0b10000000) != 0
    }

    pub fn is_signal_too_low(&self) -> bool {
        (self.detailed_status() & 0b01000000) != 0
    }

    pub fn is_temperature_out_of_range(&self) -> bool {
        (self.detailed_status() & 0b00100000) != 0
    }

    pub fn is_speed_too_high(&self) -> bool {
        (self.detailed_status() & 0b00010000) != 0
    }

    pub fn is_multiturn_counter_error(&self) -> bool {
        (self.detailed_status() & 0b00001000) != 0
    }
}

impl PositionAndStatusOuter for PositionAndDetailedStatus {
    fn inner(&self) -> &PositionAndStatusInner {
        &self.inner
    }
}

impl PrefixedResponse for PositionAndDetailedStatus {
    fn command() -> Command {
        Command::PositionRequestAndDetailedStatus
    }

    fn prefix(&self) -> u8 {
        self.inner.prefix().unwrap()[0]
    }
}

impl AsMut<[u8]> for PositionAndDetailedStatus {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleturn_position() {
        use std::io::Write;

        const PREFIX_LETTER: u8 = b'd';

        let mut pos = PositionAndDetailedStatus::new(CounterType::SingleTurn);

        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000001_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111111, 0b111111_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01111111, 0b111111_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b10000000, 0b000000_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01001000, 0b100010_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 4642);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11100001, 0b001100_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), -1972);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00101111, 0b100001_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 3041);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111011, 0b010010_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), -302);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00001001, 0b011101_00, 0b00000000])
            .unwrap();
        assert_eq!(pos.position(), 605);
    }

    #[test]
    fn test_detailed_status() {
        use std::io::Write;

        const PREFIX_LETTER: u8 = b'd';

        let mut result = PositionAndDetailedStatus::new(CounterType::SingleTurn);

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b00000000])
            .unwrap();
        assert!(!result.is_signal_too_high());
        assert!(!result.is_signal_too_low());
        assert!(!result.is_temperature_out_of_range());
        assert!(!result.is_speed_too_high());
        assert!(!result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b10000000])
            .unwrap();
        assert!(result.is_signal_too_high());
        assert!(!result.is_signal_too_low());
        assert!(!result.is_temperature_out_of_range());
        assert!(!result.is_speed_too_high());
        assert!(!result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b01000000])
            .unwrap();
        assert!(!result.is_signal_too_high());
        assert!(result.is_signal_too_low());
        assert!(!result.is_temperature_out_of_range());
        assert!(!result.is_speed_too_high());
        assert!(!result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b00100000])
            .unwrap();
        assert!(!result.is_signal_too_high());
        assert!(!result.is_signal_too_low());
        assert!(result.is_temperature_out_of_range());
        assert!(!result.is_speed_too_high());
        assert!(!result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b00010000])
            .unwrap();
        assert!(!result.is_signal_too_high());
        assert!(!result.is_signal_too_low());
        assert!(!result.is_temperature_out_of_range());
        assert!(result.is_speed_too_high());
        assert!(!result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b00001000])
            .unwrap();
        assert!(!result.is_signal_too_high());
        assert!(!result.is_signal_too_low());
        assert!(!result.is_temperature_out_of_range());
        assert!(!result.is_speed_too_high());
        assert!(result.is_multiturn_counter_error());

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0b11111000])
            .unwrap();
        assert!(result.is_signal_too_high());
        assert!(result.is_signal_too_low());
        assert!(result.is_temperature_out_of_range());
        assert!(result.is_speed_too_high());
        assert!(result.is_multiturn_counter_error());
    }
}
