#![no_std]

extern crate emlib;

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate cortex_m;

extern crate efm32hg222f64;
extern crate efm32hg222f64_hal as hal;
extern crate embedded_hal;

use emlib::*;

use embedded_hal::blocking::delay::DelayMs;
use hal::delay::Delay;
use hal::gpio::*;

fn main() {
    const CLOCK_FREQ :u32 = 14_000_000;

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = efm32hg222f64::Peripherals::take().unwrap();

    chip::init();

    cmu::oscillator_enable(cmu::Osc::LFRCO, true, true);
    cmu::clock_select_set(cmu::Clock::LFA, cmu::Select::LFRCO);

    cmu::clock_enable(cmu::Clock::CORE, true);
    cmu::clock_enable(cmu::Clock::CORELE, true);
    cmu::clock_enable(cmu::Clock::HFPER, true);

    cmu::clock_enable(cmu::Clock::GPIO, true);

    let gpio = dp.GPIO.split();
    let mut led = gpio.pf3.into_push_pull_output(); /* Led is on PF3 pin */

    let syst = cp.SYST;
    let mut delay = Delay::new(syst, CLOCK_FREQ);

    loop {
        led.toggle();

        delay.delay_ms(500u16);
    }
}

default_handler!(default_handler);

pub fn default_handler() {
    loop {}
}