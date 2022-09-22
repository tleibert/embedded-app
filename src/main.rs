#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::OutputSwitch;

// delay for 1 second
const DELAY: u32 = 50;

#[entry]
fn main() -> ! {
    // setup device peripherals
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds - all will be set to off
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    loop {
        // duplicate the original program
        // turn on each led in sequence, with a delay between leds
        for led in leds.iter_mut().rev() {
            led.on().ok();
            delay.delay_ms(DELAY);
        }

        // turn the leds off in the same sequence
        for led in leds.iter_mut().rev() {
            led.off().ok();
            delay.delay_ms(DELAY)
        }

        delay.delay_ms(DELAY);
    }
}
