#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
use cortex_m::asm;

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

#[macro_use]
extern crate efm32hg222f64;

#[macro_use]
extern crate embedded_hal as hal;

extern crate cast;

pub extern crate emlib;

pub mod delay;
pub mod gpio;