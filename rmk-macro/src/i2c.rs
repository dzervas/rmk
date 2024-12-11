//! Initialize I2C config boilerplate of RMK
//!
use quote::{format_ident, quote};
use rmk_config::toml_config::GPIOConfig;

use crate::keyboard_config::KeyboardConfig;
use crate::ChipModel;

pub(crate) fn build_i2c_config(
    chip: &ChipModel,
    gpio_config: &GPIOConfig,
) -> proc_macro2::TokenStream {
    if !gpio_config.i2c_enabled {
        return quote! {};
    }

    let sda_pin = format_ident!(
        "{}",
        gpio_config.i2c_sda.as_ref().expect("I2C SDA pin not set")
    );
    let scl_pin = format_ident!(
        "{}",
        gpio_config.i2c_scl.as_ref().expect("I2C SCL pin not set")
    );

    match chip.series {
        crate::ChipSeries::Nrf52 => {
            let freq = if let Some(f) = gpio_config.i2c_freq_khz.clone() {
                match f {
                    100 => quote! {::embassy_nrf::twim::Frequency::K100},
                    250 => quote! {::embassy_nrf::twim::Frequency::K250},
                    400 => quote! {::embassy_nrf::twim::Frequency::K400},
                    _ => panic!("Invalid I2C frequency {}", f),
                }
            } else {
                quote! {::embassy_nrf::twim::Frequency::K100}
            };

            // TODO: Check low power implementation: https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/twim_lowpower.rs
            quote! {
                let mut i2c = {
                    let mut config = ::embassy_nrf::twim::Config::default();
                    config.frequency = #freq;
                    ::defmt::info!("I2C initialized");

                    ::embassy_nrf::twim::Twim::new(
                        p.TWISPI0,
                        Irqs,
                        p.#sda_pin,
                        p.#scl_pin,
                        config)
                };
                ::embassy_nrf::interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(::embassy_nrf::interrupt::Priority::P2);
            }
        }
        crate::ChipSeries::Rp2040 => {
            let freq = if let Some(f) = gpio_config.i2c_freq_khz.clone() {
                f
            } else {
                100
            };
            quote! {
                let mut i2c = {
                    let mut config = ::embassy_rp::i2c::Config::default();
                    config.frequency = #freq * 1000;

                    ::embassy_rp::i2c::I2c::new_blocking(
                        p.I2C0,
                        p.#sda_pin,
                        p.#scl_pin,
                        config)
                }
            }
        }
        crate::ChipSeries::Stm32 => {
            let freq = if let Some(f) = gpio_config.i2c_freq_khz.clone() {
                f
            } else {
                100
            };
            quote! {
                let mut i2c = ::embassy_stm32::i2c::I2c::new_blocking(
                    p.I2C1,
                    p.#sda_pin,
                    p.#scl_pin,
                    ::embassy_stm32::time::Hertz(#freq * 1000),
                    ::embassy_stm32::i2c::Config::default()
                )
            }
        }
        _ => quote! {None},
    }
}

pub(crate) fn i2c_gpio_expander(gpio_config: &GPIOConfig) -> proc_macro2::TokenStream {
    if !gpio_config.i2c_enabled {
        return quote! {};
    }

    if gpio_config.mcp23017_enabled {
        quote! {
            let mut mcp23x17 = port_expander::Mcp23x17::new_mcp23017(i2c, false, false, false);
            let ge = mcp23x17.split();
            ::defmt::info!("MCP23017 initialized");
        }
    } else {
        quote! {}
    }
}

pub(crate) fn expand_i2c_config(keyboard_config: &KeyboardConfig) -> proc_macro2::TokenStream {
    let mut i2c_init = build_i2c_config(&keyboard_config.chip, &keyboard_config.gpio);
    let i2c_gpio_expander = i2c_gpio_expander(&keyboard_config.gpio);
    i2c_init.extend(i2c_gpio_expander);

    i2c_init
}
