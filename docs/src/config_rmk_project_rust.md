# Use Rust

If you want to customize your firmware using Rust, there're some steps to do to make the generated firmware project compile:

### 2.1 Update memory.x

`memory.x` is the linker script of Rust embedded project, it's used to define the memory layout of the microcontroller.
For most ARM Cortex-M microcontrollers, you only need to update the `LENGTH` of FLASH and RAM to your microcontroller.
You can either checkout your microcontroller's datasheet or existing Rust project of your microcontroller for it.

### 2.2 Update USB interrupt binding in `main.rs`

Next, you have to check generated `src/main.rs`, make sure that the binded USB interrupt is right. Different
microcontrollers have different types of USB peripheral, so does bind interrupt. You can check
out [Embassy's examples](https://github.com/embassy-rs/embassy/tree/main/examples) for how to bind the USB interrupt
correctly.

For example, if you're using stm32f4, there is
an [usb serial example](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/usb_serial.rs) there.
And code for binding USB interrupt is
at [line 15-17](https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/usb_serial.rs#L15-L17):

```rust
bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});
```

Don't forget to import all used items!

### 2.3 Add your own layout

The next step is to add your own keymap layout for your firmware. RMK supports [vial app](https://get.vial.today/), an
open-source cross-platform(windows/macos/linux/web) keyboard configurator. So the vial like keymap definition has to be
imported to the firmware project.

Fortunately, RMK does most of the heavy things for you, all you need to do is to create your own keymap definition and
convert it to `vial.json` following vial's doc **[here](https://get.vial.today/docs/porting-to-via.html)**, and place it
at the root of the firmware project, replacing the default one. RMK would do all the rest things for you.

### 2.4 Add your default keymap

After adding the layout of your keyboard, the default keymap should also be updated. The default keymap is defined
in `src/keymap.rs`, update keyboard matrix constants and `KEYMAP` according to your keyboard. RMK provides a bunch of
useful [macros](https://docs.rs/rmk/latest/rmk/#macros) helping you define your keymap. Check
out [keymap_configuration](https://haobogu.github.io/rmk/keymap_configuration.html) chapter for more details.

### 2.5 Define your matrix

Next, you're going to change the IO pins of keyboard matrix making RMK run on your own PCB. Generally, IO pins are
defined in `src/main.rs`. RMK will generate a helper macro to help you to define the matrix. For example, if you're
using rp2040, you can define your pins using `config_matrix_pins_rp!`:

```rust
let (input_pins, output_pins) = config_matrix_pins_rp!(
    peripherals: p,
    input: [PIN_6, PIN_7, PIN_8, PIN_9],
    output: [PIN_19, PIN_20, PIN_21]
);
```

`input` and `output` are lists of used pins, change them accordingly.

So far so good, you've done all necessary modifications of your firmware project. The next step is compiling and
flashing your firmware!