#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::iprintln;
use cortex_m_rt::entry;
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

    let primes = Primes::new(2);
    for (i, n) in primes.enumerate() {
        iprintln!(stim, "{:>6}: {}", i, n);
        // TODO: Advance LED here
    }

    // Unreachable
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

struct Primes {
    curr: u32,
}

impl Primes {
    fn new(start: u32) -> Self {
        Self { curr: start - 1 }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            self.curr += 1;
            if let Some(n) = check_prime(self.curr) {
                break Some(n);
            }
        }
    }
}

fn check_prime(n: u32) -> Option<u32> {
    let middle = (n as f32 / 2f32) as u32 + 1;

    for i in 2..middle {
        if n % i == 0 {
            return None;
        }
    }

    Some(n)
}
