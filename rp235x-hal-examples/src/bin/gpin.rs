//! # gpin External Clocks example
//!
//! This application demonstrates how to clock the processor using an external clock on GPIO20
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the top-level `README.md` file for Copyright and license details.

#![no_std]
#![no_main]

use embedded_hal_0_2::digital::v2::ToggleableOutputPin;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// To use the .MHz() function
use fugit::RateExtU32;

use rp235x_hal::clocks::ClockSource;
// Alias for our HAL crate
use rp235x_hal as hal;

// Necessary HAL types
use hal::{clocks::ClocksManager, gpin::GpIn0, gpio, Clock, Sio};

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// The external clock provided to GPIO pin 20.
const GPIN_EXTERNAL_CLOCK_FREQ_HZ: u32 = 1_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp235x_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP235x to accept an external clock on Gpio20,
/// then configures the system clock to run off this clock.
#[rp235x_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let gpin0_pin = pins.gpio20.reconfigure();
    let gpin0: GpIn0 = GpIn0::new(gpin0_pin, GPIN_EXTERNAL_CLOCK_FREQ_HZ.Hz());

    let mut clocks = ClocksManager::new(pac.CLOCKS);

    clocks
        .system_clock
        .configure_clock(&gpin0, gpin0.get_freq())
        .unwrap();

    let mut test_pin = pins.gpio0.into_push_pull_output();

    loop {
        // Continuously toggle a pin so it's possible to observe on a scope that the pico runs on
        // the externally provided frequency, and is synchronized to it.
        test_pin.toggle().unwrap();
    }
}
