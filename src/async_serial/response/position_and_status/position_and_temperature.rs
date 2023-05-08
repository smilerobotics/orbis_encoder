use super::*;
use crate::async_serial::{response::PrefixedResponse, Command};

pub struct PositionAndTemperature {
    counter_type: CounterType,
    inner: PositionAndStatusInner,
}

impl PositionAndTemperature {
    const PREFIX_SIZE: usize = 1;
    const TEMPERATURE_DATA_SIZE: usize = 2;

    pub fn new(counter_type: CounterType) -> Self {
        Self {
            counter_type,
            inner: PositionAndStatusInner::new(
                counter_type,
                Self::PREFIX_SIZE,
                Self::TEMPERATURE_DATA_SIZE,
            ),
        }
    }

    pub fn counter_type(&self) -> CounterType {
        self.counter_type
    }

    pub fn temperature(&self) -> f64 {
        i16::from_be_bytes(self.inner.postfix().unwrap().try_into().unwrap()) as f64 / 10.0
    }
}

impl PositionAndStatusOuter for PositionAndTemperature {
    fn inner(&self) -> &PositionAndStatusInner {
        &self.inner
    }
}

impl PrefixedResponse for PositionAndTemperature {
    fn command() -> Command {
        Command::PositionRequestAndTemperature
    }

    fn prefix(&self) -> u8 {
        self.inner.prefix().unwrap()[0]
    }
}

impl AsMut<[u8]> for PositionAndTemperature {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_singleturn_position() {
        use std::io::Write;

        const PREFIX_LETTER: u8 = b't';

        let mut pos = PositionAndTemperature::new(CounterType::SingleTurn);

        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000001_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111111, 0b111111_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01111111, 0b111111_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b10000000, 0b000000_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01001000, 0b100010_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 4642);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11100001, 0b001100_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), -1972);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00101111, 0b100001_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 3041);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111011, 0b010010_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), -302);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00001001, 0b011101_00, 0x00, 0x00])
            .unwrap();
        assert_eq!(pos.position(), 605);
    }

    #[test]
    fn test_temperature() {
        use std::io::Write;

        const PREFIX_LETTER: u8 = b't';

        let mut result = PositionAndTemperature::new(CounterType::SingleTurn);

        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0x00, 0x00])
            .unwrap();
        assert_approx_eq!(result.temperature(), 0.0);
        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0x00, 0x64])
            .unwrap();
        assert_approx_eq!(result.temperature(), 10.0);
        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0x01, 0x0D])
            .unwrap();
        assert_approx_eq!(result.temperature(), 26.9);
        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0x30, 0x39])
            .unwrap();
        assert_approx_eq!(result.temperature(), 1234.5);
        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0xFF, 0x9C])
            .unwrap();
        assert_approx_eq!(result.temperature(), -10.0);
        result
            .as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00, 0xF5, 0x54])
            .unwrap();
        assert_approx_eq!(result.temperature(), -273.2);
    }
}
