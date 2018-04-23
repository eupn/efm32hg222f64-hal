use core::marker::PhantomData;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

use emlib::*;

macro_rules! gpio {
    ([
        $($PORT:path, $PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod gpio {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;
            use emlib;
            use emlib::gpio::Port;

            use efm32hg222f64;

            use super::{
                Alternate, Floating, Input, GpioExt,

                // OpenDrain,
                Output,

                // PullDown, PullUp,
                PushPull,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for efm32hg222f64::GPIO {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi { port: $PORT, i: $i, _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    port: Port,
                    i: u32,
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as an alternate function push pull output pin
                    pub fn into_alternate_push_pull(
                        self,
                    ) -> $PXi<Alternate<PushPull>> {

                        $PXi { port: $PORT, i: $i, _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                    ) -> $PXi<Input<Floating>> {
                        emlib::gpio::pin_mode_set(self.port, $i, emlib::gpio::Mode::Input, 0);

                        $PXi { port: $PORT, i: $i, _mode: PhantomData }
                    }

                    // /// Configures the pin to operate as a pulled down input pin
                    // pub fn into_pull_down_input(
                    //     self,
                    //     moder: &mut MODER,
                    //     pupdr: &mut PUPDR,
                    // ) -> $PXi<Input<PullDown>> {
                    //     let offset = 2 * $i;

                    //     // input mode
                    //     moder
                    //         .moder()
                    //         .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                    //     // pull-down
                    //     pupdr.pupdr().modify(|r, w| unsafe {
                    //         w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                    //     });

                    //     $PXi { _mode: PhantomData }
                    // }

                    // /// Configures the pin to operate as a pulled up input pin
                    // pub fn into_pull_up_input(
                    //     self,
                    //     moder: &mut MODER,
                    //     pupdr: &mut PUPDR,
                    // ) -> $PXi<Input<PullUp>> {
                    //     let offset = 2 * $i;

                    //     // input mode
                    //     moder
                    //         .moder()
                    //         .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                    //     // pull-up
                    //     pupdr.pupdr().modify(|r, w| unsafe {
                    //         w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                    //     });

                    //     $PXi { _mode: PhantomData }
                    // }

                    // /// Configures the pin to operate as an open drain output pin
                    // pub fn into_open_drain_output(
                    //     self,
                    //     moder: &mut MODER,
                    //     otyper: &mut OTYPER,
                    // ) -> $PXi<Output<OpenDrain>> {
                    //     let offset = 2 * $i;

                    //     // general purpose output mode
                    //     let mode = 0b01;
                    //     moder.moder().modify(|r, w| unsafe {
                    //         w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                    //     });

                    //     // open drain output
                    //     otyper
                    //         .otyper()
                    //         .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << $i)) });

                    //     $PXi { _mode: PhantomData }
                    // }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                    ) -> $PXi<Output<PushPull>> {
                        emlib::gpio::pin_mode_set(self.port, $i, emlib::gpio::Mode::PushPullDrive, 0);

                        $PXi { port: $PORT, i: $i, _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    pub fn toggle(&mut self) {
                        emlib::gpio::pin_out_toggle(self.port, $i)
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        emlib::gpio::pin_in_get(self.port, self.i) == 0
                    }

                    fn set_high(&mut self) {
                        emlib::gpio::pin_out_set(self.port, self.i);
                    }

                    fn set_low(&mut self) {
                        emlib::gpio::pin_out_clear(self.port, self.i);
                    }
                }
            )+
        }
    }
}

/// EFM32HG222 boilerplate for type-safe GPIO pins
gpio!([
    emlib::gpio::Port::A, PA0: (pa0, 0, Input<Floating>),
    emlib::gpio::Port::A, PA1: (pa1, 1, Input<Floating>),
    emlib::gpio::Port::A, PA2: (pa2, 2, Input<Floating>),

    emlib::gpio::Port::A, PA8: (pa8, 8, Input<Floating>),
    emlib::gpio::Port::A, PA9: (pa9, 9, Input<Floating>),
    emlib::gpio::Port::A, PA10: (pa10, 10, Input<Floating>),

    emlib::gpio::Port::B, PB7: (pb7, 7, Input<Floating>),
    emlib::gpio::Port::B, PB8: (pb8, 8, Input<Floating>),

    emlib::gpio::Port::B, PB13: (pb13, 13, Input<Floating>),
    emlib::gpio::Port::B, PB14: (pb14, 14, Input<Floating>),

    emlib::gpio::Port::C, PC0: (pc0, 0, Input<Floating>),
    emlib::gpio::Port::C, PC1: (pc1, 1, Input<Floating>),
    emlib::gpio::Port::C, PC2: (pc2, 2, Input<Floating>),
    emlib::gpio::Port::C, PC3: (pc3, 3, Input<Floating>),
    emlib::gpio::Port::C, PC4: (pc4, 4, Input<Floating>),
    emlib::gpio::Port::C, PC8: (pc8, 8, Input<Floating>),
    emlib::gpio::Port::C, PC9: (pc9, 9, Input<Floating>),
    emlib::gpio::Port::C, PC10: (pc10, 10, Input<Floating>),
    emlib::gpio::Port::C, PC11: (pc11, 11, Input<Floating>),
    emlib::gpio::Port::C, PC13: (pc13, 13, Input<Floating>),
    emlib::gpio::Port::C, PC14: (pc14, 14, Input<Floating>),
    emlib::gpio::Port::C, PC15: (pc15, 15, Input<Floating>),

    emlib::gpio::Port::D, PD4: (pd4, 4, Input<Floating>),
    emlib::gpio::Port::D, PD5: (pd5, 5, Input<Floating>),
    emlib::gpio::Port::D, PD6: (pd6, 6, Input<Floating>),
    emlib::gpio::Port::D, PD7: (pd7, 7, Input<Floating>),

    emlib::gpio::Port::F, PF0: (pf0, 0, Input<Floating>),
    emlib::gpio::Port::F, PF1: (pf1, 1, Input<Floating>),
    emlib::gpio::Port::F, PF2: (pf2, 2, Input<Floating>),
    emlib::gpio::Port::F, PF3: (pf3, 3, Input<Floating>),
    emlib::gpio::Port::F, PF4: (pf4, 4, Input<Floating>),
    emlib::gpio::Port::F, PF5: (pf5, 5, Input<Floating>),

    emlib::gpio::Port::E, PE10: (pe10, 10, Input<Floating>),
    emlib::gpio::Port::E, PE11: (pe11, 11, Input<Floating>),
    emlib::gpio::Port::E, PE12: (pe12, 12, Input<Floating>),
    emlib::gpio::Port::E, PE13: (pe13, 13, Input<Floating>),
]);