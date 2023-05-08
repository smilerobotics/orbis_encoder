use std::f64::consts::PI;

use crate::{CounterType, COUNTS_PER_REVOLUTION};

pub trait PositionAndStatus {
    fn multiturn_count(&self) -> Option<i16>;
    fn position(&self) -> i16;
    fn is_error(&self) -> bool;
    fn is_warning(&self) -> bool;

    fn angle_rad(&self) -> f64 {
        2.0 * PI
            * (self.multiturn_count().map_or(0.0, |count| count as f64)
                + self.position() as f64 / COUNTS_PER_REVOLUTION as f64)
    }
}

pub struct PositionAndStatusInner {
    buf: Vec<u8>,
    prefix_size: usize,
    multiturn_data_offset: Option<usize>,
    position_data_offset: usize,
    status_data_offset: usize,
    postfix_offset: Option<usize>,
}

impl PositionAndStatusInner {
    const MULTITURN_DATA_SIZE: usize = 2;
    const POSITION_DATA_SIZE: usize = 2;

    fn new(counter_type: CounterType, prefix_size: usize, postfix_size: usize) -> Self {
        let multiturn_data_offset = match counter_type {
            CounterType::SingleTurn => None,
            CounterType::MultiTurn => Some(prefix_size),
        };
        let position_data_offset =
            multiturn_data_offset.map_or(prefix_size, |u| u + Self::MULTITURN_DATA_SIZE);
        let status_data_offset = position_data_offset + 1;
        let postfix_offset = if postfix_size == 0 {
            None
        } else {
            Some(position_data_offset + Self::POSITION_DATA_SIZE)
        };

        let buf = vec![
            0;
            postfix_offset.map_or(position_data_offset + Self::POSITION_DATA_SIZE, |u| u
                + postfix_size)
        ];

        Self {
            buf,
            prefix_size,
            multiturn_data_offset,
            position_data_offset,
            status_data_offset,
            postfix_offset,
        }
    }

    fn multiturn_count(&self) -> Option<i16> {
        let offset = self.multiturn_data_offset?;
        Some(i16::from_be_bytes(
            self.buf[offset..offset + Self::MULTITURN_DATA_SIZE]
                .try_into()
                .unwrap(),
        ))
    }

    fn position(&self) -> i16 {
        let offset = self.position_data_offset;
        i16::from_be_bytes(
            self.buf[offset..offset + Self::POSITION_DATA_SIZE]
                .try_into()
                .unwrap(),
        ) >> 2
    }

    fn is_error(&self) -> bool {
        (self.buf[self.status_data_offset] & 0b00000010) == 0
    }

    fn is_warning(&self) -> bool {
        (self.buf[self.status_data_offset] & 0b00000001) == 0
    }

    fn prefix(&self) -> Option<&[u8]> {
        if self.prefix_size == 0 {
            None
        } else {
            Some(&self.buf[..self.prefix_size])
        }
    }

    fn postfix(&self) -> Option<&[u8]> {
        self.postfix_offset.map(|offset| &self.buf[offset..])
    }
}

impl AsMut<[u8]> for PositionAndStatusInner {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut()
    }
}

pub trait PositionAndStatusOuter {
    fn inner(&self) -> &PositionAndStatusInner;
}

impl<T: PositionAndStatusOuter> PositionAndStatus for T {
    fn multiturn_count(&self) -> Option<i16> {
        self.inner().multiturn_count()
    }

    fn position(&self) -> i16 {
        self.inner().position()
    }

    fn is_error(&self) -> bool {
        self.inner().is_error()
    }

    fn is_warning(&self) -> bool {
        self.inner().is_warning()
    }
}

mod position;
pub use position::Position;

mod short_position;
pub use short_position::ShortPosition;

mod position_and_temperature;
pub use position_and_temperature::PositionAndTemperature;

mod position_and_detailed_status;
pub use position_and_detailed_status::PositionAndDetailedStatus;

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_singleturn_position() {
        use std::io::Write;

        let mut pos = PositionAndStatusInner::new(CounterType::SingleTurn, 0, 0);

        pos.as_mut().write_all(&[0b00000000, 0b000000_00]).unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut().write_all(&[0b00000000, 0b000000_01]).unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut().write_all(&[0b00000000, 0b000000_10]).unwrap();
        assert_eq!(pos.position(), 0);
        pos.as_mut().write_all(&[0b00000000, 0b000000_11]).unwrap();
        assert_eq!(pos.position(), 0);

        pos.as_mut().write_all(&[0b00000000, 0b000001_00]).unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut().write_all(&[0b00000000, 0b000001_01]).unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut().write_all(&[0b00000000, 0b000001_10]).unwrap();
        assert_eq!(pos.position(), 1);
        pos.as_mut().write_all(&[0b00000000, 0b000001_11]).unwrap();
        assert_eq!(pos.position(), 1);

        pos.as_mut().write_all(&[0b11111111, 0b111111_00]).unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut().write_all(&[0b11111111, 0b111111_01]).unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut().write_all(&[0b11111111, 0b111111_10]).unwrap();
        assert_eq!(pos.position(), -1);
        pos.as_mut().write_all(&[0b11111111, 0b111111_11]).unwrap();
        assert_eq!(pos.position(), -1);

        pos.as_mut().write_all(&[0b01111111, 0b111111_00]).unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut().write_all(&[0b01111111, 0b111111_01]).unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut().write_all(&[0b01111111, 0b111111_10]).unwrap();
        assert_eq!(pos.position(), 8191);
        pos.as_mut().write_all(&[0b01111111, 0b111111_11]).unwrap();
        assert_eq!(pos.position(), 8191);

        pos.as_mut().write_all(&[0b10000000, 0b000000_00]).unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut().write_all(&[0b10000000, 0b000000_01]).unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut().write_all(&[0b10000000, 0b000000_10]).unwrap();
        assert_eq!(pos.position(), -8192);
        pos.as_mut().write_all(&[0b10000000, 0b000000_11]).unwrap();
        assert_eq!(pos.position(), -8192);

        pos.as_mut().write_all(&[0b01001000, 0b100010_00]).unwrap();
        assert_eq!(pos.position(), 4642);
        pos.as_mut().write_all(&[0b11100001, 0b001100_00]).unwrap();
        assert_eq!(pos.position(), -1972);
        pos.as_mut().write_all(&[0b00101111, 0b100001_00]).unwrap();
        assert_eq!(pos.position(), 3041);
        pos.as_mut().write_all(&[0b11111011, 0b010010_00]).unwrap();
        assert_eq!(pos.position(), -302);
        pos.as_mut().write_all(&[0b00001001, 0b011101_00]).unwrap();
        assert_eq!(pos.position(), 605);
    }

    struct MockPositionAndStatus {
        multiturn_count: Option<i16>,
        position: i16,
    }

    impl PositionAndStatus for MockPositionAndStatus {
        fn multiturn_count(&self) -> Option<i16> {
            self.multiturn_count
        }

        fn position(&self) -> i16 {
            self.position
        }

        fn is_error(&self) -> bool {
            false
        }

        fn is_warning(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_angle() {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: 0,
        };
        assert_approx_eq!(pos.angle_rad(), 0.0);

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: 2048,
        };
        assert_approx_eq!(pos.angle_rad(), FRAC_PI_4);

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: 4096,
        };
        assert_approx_eq!(pos.angle_rad(), FRAC_PI_2);

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: -4096,
        };
        assert_approx_eq!(pos.angle_rad(), -FRAC_PI_2);

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: -8192,
        };
        assert_approx_eq!(pos.angle_rad(), -PI);

        let pos = MockPositionAndStatus {
            multiturn_count: None,
            position: -2048,
        };
        assert_approx_eq!(pos.angle_rad(), -FRAC_PI_4);
    }
}
