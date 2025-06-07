
use super::config::*;
use embedded_hal::i2c::I2c;

pub struct XL9555<I2C> {
    i2c: I2C,
    dev_addr: u8
}

impl<I2C> XL9555<I2C>
where 
    I2C: I2c
{
    pub fn init(
        i2c: I2C,
        (a1, a2, a0): (bool, bool, bool),
    ) -> Self {
        let mut xl9555 = Self {
            i2c,
            dev_addr: XL9555_BASE_ADDR | ((a2 as u8) << 2) | ((a1 as u8) << 1) | (a0 as u8),
        };
        let mut buffer = [0u8; 2];
        xl9555.read_byte(XL9555_INPUT_PORT0_REG, &mut buffer).unwrap();
        xl9555
    }
    /// Read the 16-bit IO value of XL9555
    #[inline]
    fn read_byte(&mut self, reg_addr: u8, buffer: &mut [u8; 2]) -> Result<(), I2C::Error> {
        let write_buffer = [reg_addr];
        self.i2c
            .write_read(self.dev_addr, &write_buffer[..], buffer)
    }
    /// Write the 16-bit IO value to XL9555
    #[inline]
    fn write_byte(&mut self, reg_addr: u8, buffer: &[u8; 2]) -> Result<(), I2C::Error> {
        let mut write_buffer = [0u8; 3];
        write_buffer[0] = reg_addr;
        write_buffer[1..3].copy_from_slice(buffer);
        self.i2c.write(self.dev_addr, &write_buffer)
    }
    /// Configures the IO pin modes of the XL9555 chip.
    ///
    /// This function takes a 16-bit unsigned integer `value`, where each bit corresponds to the mode of an IO pin on the XL9555 chip.
    /// A bit value of `1` indicates that the corresponding pin should be configured as an input, while a value of `0` configures the pin as an output.  
    ///
    /// # Parameters
    /// - `value`: A 16-bit unsigned integer representing the configuration of the IO pins. `1` indicates input mode, `0` indicates output mode.
    ///
    /// # Return Value
    /// - `Ok(())`: Indicates successful configuration.
    /// - `Err(I2C::Error)`: Indicates a failure in the configuration, returning an error.
    ///
    pub fn xl9555_ioconfig(&mut self, value: u16) -> Result<(), I2C::Error> {
        let mut buffer = [0u8; 2];
        buffer[0] = value as u8;
        buffer[1] = (value >> 8) as u8;
        self.write_byte(XL9555_CONFIG_PORT0_REG, &buffer)
    }
    /// Sets the output value of a specific IO pin.
    ///
    /// # Parameters
    /// - `pin`: The IO pin to modify.
    /// - `value`: `true` to set the pin high, `false` to set it low.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(I2C::Error)` on failure.
    pub fn set_value(&mut self, pin: Pin, value: bool) -> Result<(), I2C::Error> {
        let pin = pin.bits();
        let mut buffer = [0u8; 2];
        self.read_byte(XL9555_OUTPUT_PORT0_REG, &mut buffer)?;
        if pin <= 0x0080 {
            if value {
                buffer[0] |= pin as u8;
            } else {
                buffer[0] &= !(pin as u8);
            }
        } else {
            if value {
                buffer[1] |= (pin >> 8) as u8;
            } else {
                buffer[1] &= !((pin >> 8) as u8);
            }
        }

        self.write_byte(XL9555_OUTPUT_PORT0_REG, &buffer)
    }
    /// Reads the value of a specific IO pin.
    ///
    /// # Parameters
    /// - `pin`: The IO pin to read.
    ///
    /// # Returns
    /// - `Ok(true)` if the pin is high.
    /// - `Ok(false)` if the pin is low.
    /// - `Err(I2C::Error)` if a read error occurs.
    pub fn read_value(&mut self, pin: Pin) -> Result<bool, I2C::Error> {
        let pin = pin.bits();
        let mut buffer = [0u8; 2];
        self.read_byte(XL9555_INPUT_PORT0_REG, &mut buffer)?;
        if pin <= 0x0080 {
            Ok((buffer[0] & pin as u8) != 0)
        } else {
            Ok((buffer[1] & pin as u8) != 0)
        }
    }
    /// Reads all 16 IO values of the XL9555.
    ///
    /// # Returns
    /// - `Ok(u16)`: The 16-bit value representing all IO pin states.
    /// - `Err(I2C::Error)`: If an error occurs during the read operation.
    pub fn read_all_value(&mut self) -> Result<u16, I2C::Error> {
        let mut buffer = [0u8; 2];
        self.read_byte(XL9555_INPUT_PORT0_REG, &mut buffer)?;
        Ok(((buffer[1] as u16) << 8) + (buffer[0] as u16))
    }
}