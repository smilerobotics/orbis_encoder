use super::Command;
use crate::COUNTS_PER_REVOLUTION;

#[derive(Clone, Copy, Debug)]
pub enum ProgrammingCommand {
    PositionOffsetSetting(i16),
    MultiturnCounterSetting(i16),
    BaudRateSetting(u32),
    ContinuousResponseSetting {
        auto_start: bool,
        command: Command,
        period_micros: u16,
    },
    ContinuousResponseStart,
    ContinuousResponseStop,
    ConfigurationParametersSave,
    ConfigurationParametersReset,
}

impl ProgrammingCommand {
    pub(crate) fn to_byte(self) -> u8 {
        match self {
            Self::PositionOffsetSetting(_) => b'Z',
            Self::MultiturnCounterSetting(_) => b'M',
            Self::BaudRateSetting(_) => b'B',
            Self::ContinuousResponseSetting {
                auto_start: _,
                command: _,
                period_micros: _,
            } => b'T',
            Self::ContinuousResponseStart => b'S',
            Self::ContinuousResponseStop => b'P',
            Self::ConfigurationParametersSave => b'c',
            Self::ConfigurationParametersReset => b'r',
        }
    }

    pub(crate) fn additional_data(&self) -> Option<Vec<u8>> {
        match self {
            Self::PositionOffsetSetting(offset) => {
                let offset = if *offset >= 0 {
                    *offset as u32
                } else {
                    (*offset + COUNTS_PER_REVOLUTION as i16) as u32
                };
                let mut data = Vec::new();
                data.extend(u32::to_be_bytes(offset));
                Some(data)
            }
            Self::MultiturnCounterSetting(count) => {
                let mut data = vec![0; 2];
                data.extend(i16::to_be_bytes(*count));
                Some(data)
            }
            Self::BaudRateSetting(baud_rate) => {
                let mut data = Vec::new();
                data.extend(u32::to_be_bytes(*baud_rate));
                Some(data)
            }
            Self::ContinuousResponseSetting {
                auto_start,
                command,
                period_micros,
            } => {
                let mut data = Vec::new();
                data.push(u8::from(*auto_start));
                data.push(command.to_byte());
                data.append(&mut u16::to_be_bytes(*period_micros).to_vec());
                Some(data)
            }
            _ => None,
        }
    }
}
