// buggy: https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+derive_partial_eq_without_eq
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::unusual_byte_groupings)]

const BITS_PER_REVOLUTION: usize = 14;
const COUNTS_PER_REVOLUTION: usize = 2 << (BITS_PER_REVOLUTION - 1);

pub mod async_serial;
mod counter_type;
pub mod error;

pub use counter_type::CounterType;
