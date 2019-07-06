#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::iprintln;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use f3::{
    hal::{delay::Delay, prelude::*, stm32f30x},
    led::Leds,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim[0];

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);

    hprintln!("Hello, world!").unwrap();
    iprintln!(stim, "Hello, world!");

    let n = leds.len();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();

            delay.delay_ms(100_u8);
        }
    }
}
