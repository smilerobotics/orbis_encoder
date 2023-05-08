use std::time::Duration;

use orbis_encoder::{async_serial::*, CounterType};

const DEFAULT_DEVICE_FILE_PATH: &str = "/dev/ttyUSB0";
const BAUD_RATE: u32 = 1_000_000;
const TIMEOUT: Duration = Duration::from_millis(1000);

const DEFAULT_CYCLE_TIME_MICROS: u16 = 10_000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("p", "port", "serial port path", "PATH");
    opts.optopt("c", "cycle_time", "cycle time (micro sec)", "CYCLE_TIME");
    let matches = opts.parse(&args[1..]).unwrap();
    let path = matches
        .opt_str("p")
        .unwrap_or_else(|| DEFAULT_DEVICE_FILE_PATH.to_owned());
    let cycle_time = matches
        .opt_str("c")
        .map_or_else(|| DEFAULT_CYCLE_TIME_MICROS, |s| s.parse().unwrap());

    let mut port = Port::try_new(path, BAUD_RATE, TIMEOUT).unwrap();

    port.send_programming_command(&ProgrammingCommand::ContinuousResponseStop)
        .unwrap();

    port.send_programming_command(&ProgrammingCommand::ContinuousResponseSetting {
        auto_start: false,
        command: Command::PositionRequest,
        period_micros: cycle_time,
    })
    .unwrap();

    port.send_programming_command(&ProgrammingCommand::ContinuousResponseStart)
        .unwrap();

    loop {
        let mut position = Position::new(CounterType::SingleTurn);
        port.receive(&mut position).expect("failed to receive");

        assert!(position.is_valid_prefix());
        assert!(!position.is_error());
        assert!(!position.is_warning());

        println!(
            "position: {}, angle: {}rad",
            position.position(),
            position.angle_rad()
        );
    }
}
