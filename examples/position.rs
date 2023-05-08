use std::time::Duration;

use orbis_encoder::{async_serial::*, CounterType};

const DEFAULT_DEVICE_FILE_PATH: &str = "/dev/ttyUSB0";
const BAUD_RATE: u32 = 1_000_000;
const TIMEOUT: Duration = Duration::from_millis(1000);

const INTERVAL: Duration = Duration::from_millis(20);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("p", "port", "serial port path", "PATH");
    let matches = opts.parse(&args[1..]).unwrap();
    let path = matches
        .opt_str("p")
        .unwrap_or_else(|| DEFAULT_DEVICE_FILE_PATH.to_owned());

    let mut port = Port::try_new(path, BAUD_RATE, TIMEOUT).unwrap();

    let command = Command::PositionRequest;

    loop {
        port.send_command(&command).expect("failed to send");

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

        std::thread::sleep(INTERVAL);
    }
}
