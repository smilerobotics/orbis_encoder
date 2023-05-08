// buggy: https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+derive_partial_eq_without_eq
#![allow(clippy::derive_partial_eq_without_eq)]

use std::time::Duration;

use orbis_encoder::async_serial::*;

const DEFAULT_DEVICE_FILE_PATH: &str = "/dev/ttyUSB0";
const BAUD_RATE: u32 = 1_000_000;
const TIMEOUT: Duration = Duration::from_millis(1000);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("p", "port", "serial port path", "PATH");
    let matches = opts.parse(&args[1..]).unwrap();
    let path = matches
        .opt_str("p")
        .unwrap_or_else(|| DEFAULT_DEVICE_FILE_PATH.to_owned());

    let mut port = Port::try_new(path, BAUD_RATE, TIMEOUT).unwrap();

    port.send_programming_command(&ProgrammingCommand::ContinuousResponseStop)
        .unwrap();

    let command = Command::SerialNumber;
    port.send_command(&command).expect("failed to send");

    let mut serial_number = SerialNumber::new();
    port.receive(&mut serial_number).expect("failed to receive");

    assert!(serial_number.is_valid_prefix());

    println!("{serial_number}");
}
