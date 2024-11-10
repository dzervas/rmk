use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use embedded_hal::i2c::I2c;
use mcp23017::{PinMode, MCP23017};

pub enum Bank {
    A,
    B,
}

pub struct Pin {
    addr: u8,
    bank: Bank,
    pin: u8,
}

impl Pin {
    pub fn new(addr: u8, bank: Bank, pin: u8) -> Self {
        Self { addr, bank, pin }
    }
}

pub struct Input<I2C: I2c> {
    inst: MCP23017<I2C>,
    pin: Pin,
}

impl<I2C: I2c> ErrorType for Input<I2C> {
    type Error = Infallible;
}

impl<I2C: I2c> Input<I2C> {
    pub fn new(i2c: I2C, pin: Pin) -> Result<Self, I2C::Error> {
        let mut inst = MCP23017::new(i2c, pin.addr).unwrap();
        inst.pin_mode(pin_number(&pin), PinMode::INPUT)?;
        Ok(Self { inst, pin })
    }
}

impl<I2C: I2c> InputPin for Input<I2C> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.inst.digital_read(pin_number(&self.pin)).unwrap())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|v| !v)
    }
}

pub struct Output<I2C: I2c> {
    inst: MCP23017<I2C>,
    pin: Pin,
}

impl<I2C: I2c> ErrorType for Output<I2C> {
    type Error = Infallible;
}

impl<I2C: I2c> Output<I2C> {
    pub fn new(i2c: I2C, pin: Pin) -> Result<Self, I2C::Error> {
        let mut inst = MCP23017::new(i2c, pin.addr).unwrap();
        inst.pin_mode(pin_number(&pin), PinMode::OUTPUT)?;
        Ok(Self { inst, pin })
    }
}

impl<I2C: I2c> OutputPin for Output<I2C> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self
            .inst
            .digital_write(pin_number(&self.pin), true)
            .unwrap())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self
            .inst
            .digital_write(pin_number(&self.pin), false)
            .unwrap())
    }
}

fn pin_number(pin: &Pin) -> u8 {
    match pin.bank {
        Bank::A => pin.pin,
        Bank::B => pin.pin + 8,
    }
}
