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

    let command = Command::SelfCalibrationStatusRequest;
    port.send_command(&command).expect("failed to send");

    let mut status = SelfCalibrationStatus::new();
    port.receive(&mut status).expect("failed to receive");

    assert!(status.is_valid_prefix());

    println!(
        "Calculated parameters out of range: {}",
        status.is_out_of_range()
    );
    println!("Timeout: {}", status.is_timeout());
    println!("Counter: {}", status.counter());
}
