#![no_std]

pub mod driver;
pub mod io;
#[allow(dead_code)]
mod config;

pub use config::Pin;
pub use embedded_hal::digital::PinState;
