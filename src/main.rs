#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::iprintln;
use cortex_m_rt::entry;
use f3::{
    hal::{prelude::*, stm32f30x},
    led::Leds,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim[0];

    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut leds = Leds::new(gpioe);
    let n = leds.len();

    let mut primes = Primes::new(2).enumerate();

    loop {
        for curr in 0..n {
            let (i, nth_prime) = primes.next().unwrap();
            iprintln!(stim, "{:>6}: {}", i, nth_prime);

            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();
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
