use core::cell::RefCell;

use embedded_hal::{digital::{ErrorType, OutputPin, PinState}, i2c::I2c};

use super::{driver::XL9555, config::Pin};

/// Represents a single output pin on the XL9555.
pub struct Output<'d, I2C> {
    device: &'d RefCell<XL9555<I2C>>,
    pin: Pin
}

impl<'d, I2C> Output<'d, I2C>
where 
    I2C: I2c
{
    /// Creates a new output pin and sets its initial state.
    ///
    /// # Parameters
    /// - `xl9555`: A reference to the shared XL9555 driver instance.
    /// - `pin`: The specific pin to configure as an output.
    /// - `level`: The initial state of the pin (`PinState::High` or `PinState::Low`).
    ///
    /// # Panics
    /// This function will panic if the underlying I2C communication fails.
    pub fn new(xl9555: &'d RefCell<XL9555<I2C>>, pin: Pin, level: PinState) -> Self {
        if level == PinState::High {
            xl9555.borrow_mut().set_value(pin, true).unwrap();
        } else {
            xl9555.borrow_mut().set_value(pin, false).unwrap();
        }
        let output = Self {
            device: xl9555,
            pin
        };
        output
    }
    /// Sets the output pin to a low state.
    ///
    /// # Returns
    /// - `Ok(())` if the operation was successful.
    /// - `Err(Error::I2cError)` if there was an I2C communication error.
    #[inline]
    pub fn set_low(&mut self) -> Result<(), Error> {
        self.device.borrow_mut().set_value(self.pin, false).map_err(|_| Error::I2cError)
    }
    /// Sets the output pin to a high state.
    ///
    /// # Returns
    /// - `Ok(())` if the operation was successful.
    /// - `Err(Error::I2cError)` if there was an I2C communication error.
    #[inline]
    pub fn set_high(&mut self) -> Result<(), Error> {
        self.device.borrow_mut().set_value(self.pin, true).map_err(|_| Error::I2cError)
    }
}

impl<'d, I2C> ErrorType for Output<'d, I2C>
where 
    I2C: I2c
{
    type Error = Error;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error{
    I2cError
}

impl embedded_hal::digital::Error for Error {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl<'d, I2C> OutputPin for Output<'d, I2C> 
where 
    I2C: I2c
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high()
    }
}
