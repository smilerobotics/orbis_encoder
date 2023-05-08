use super::*;
use crate::async_serial::{response::PrefixedResponse, Command};

pub struct Position {
    counter_type: CounterType,
    inner: PositionAndStatusInner,
}

impl Position {
    const ADDITIONAL_DATA_SIZE: usize = 0;
    const PREFIX_SIZE: usize = 1;

    pub fn new(counter_type: CounterType) -> Self {
        Self {
            counter_type,
            inner: PositionAndStatusInner::new(
                counter_type,
                Self::PREFIX_SIZE,
                Self::ADDITIONAL_DATA_SIZE,
            ),
        }
    }

    pub fn counter_type(&self) -> CounterType {
        self.counter_type
    }
}

impl PositionAndStatusOuter for Position {
    fn inner(&self) -> &PositionAndStatusInner {
        &self.inner
    }
}

impl PrefixedResponse for Position {
    fn command() -> Command {
        Command::PositionRequest
    }

    fn prefix(&self) -> u8 {
        self.inner.prefix().unwrap()[0]
    }
}

impl AsMut<[u8]> for Position {
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

        const PREFIX_LETTER: u8 = b'1';

        let mut pos = Position::new(CounterType::SingleTurn);

        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000000_00])
            .unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00000000, 0b000001_00])
            .unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111111, 0b111111_00])
            .unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01111111, 0b111111_00])
            .unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b10000000, 0b000000_00])
            .unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b01001000, 0b100010_00])
            .unwrap();
        assert_eq!(pos.position(), 4642);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11100001, 0b001100_00])
            .unwrap();
        assert_eq!(pos.position(), -1972);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00101111, 0b100001_00])
            .unwrap();
        assert_eq!(pos.position(), 3041);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b11111011, 0b010010_00])
            .unwrap();
        assert_eq!(pos.position(), -302);
        pos.as_mut()
            .write_all(&[PREFIX_LETTER, 0b00001001, 0b011101_00])
            .unwrap();
        assert_eq!(pos.position(), 605);
    }
}
