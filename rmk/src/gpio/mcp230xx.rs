use embedded_hal::digital::{Error, ErrorType, InputPin, OutputPin};
use embedded_hal::i2c::I2c;
use mcp23017::MCP23017;

pub struct Input<E, I2C: I2c<Error = E>> {
    inst: MCP23017<I2C>,
    pin: u8,
    bank: u8,
}

impl<E: Error, I2C: I2c<Error = E>> InputPin for Input<E, I2C> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.inst.digital_read(self.pin)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let is_high = self.is_high()?;
        Ok(!is_high)
    }
}

impl<E: Error, I2C: I2c<Error = E>> ErrorType for Input<E, I2C> {
    type Error = E;
}

pub struct Output<E, I2C: I2c<Error = E>> {
    inst: MCP23017<I2C>,
    pin: u8,
    bank: u8,
}

impl<E: Error, I2C: I2c<Error = E>> OutputPin for Output<E, I2C> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.inst.digital_write(self.pin, true)
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.inst.digital_write(self.pin, false)
    }
}

impl<E: Error, I2C: I2c<Error = E>> ErrorType for Output<E, I2C> {
    type Error = E;
}
