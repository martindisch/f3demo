#![no_std]

use libm::F32Ext;

pub struct Primes {
    next: u32,
}

impl Primes {
    pub fn new(start: u32) -> Primes {
        Primes { next: start }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            let curr = self.next;
            self.next += 1;
            if let Some(n) = check_prime(curr) {
                break Some(n);
            }
        }
    }
}

fn check_prime(n: u32) -> Option<u32> {
    let sqrt = (n as f32).sqrt() as u32 + 1;

    for i in 2..sqrt {
        if n % i == 0 {
            return None;
        }
    }

    Some(n)
}
