# XL9555 I/O Expander Rust Driver
A no_std Rust driver for the XL9555, a 16-bit I/O expander with an I2C interface. This crate is built upon the embedded-hal traits.

## Overview
The XL9555 is a CMOS device that provides 16-bit general-purpose parallel input/output (GPIO) expansion for I2C-bus applications.   

This driver allows you to:   
- Configure individual pins as inputs or outputs.   
- Read the state of input pins.   
- Set the state of output pins.   
- Use the embedded-hal digital V2 traits for GPIO control.   

## Feature
- no_std compatible, making it suitable for bare-metal and embedded systems.
- I2C communication interface.
- Individual pin control for both input and output.
- Read and write to all 16 pins in a single operation.
- Implements embedded-hal's OutputPin trait for easy integration with other drivers and applications.
- Address selection using the A0, A1, and A2 hardware pins.

## Installation
Add this to your `Cargo.toml`:   
```
[dependencies]
xl9555 = { git = "https://github.com/KaidRommel/xl9555-rs.git" }
```
