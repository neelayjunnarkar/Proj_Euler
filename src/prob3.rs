// Largest Prime Factor

use std::num::*;

pub fn run() -> String {

    let mut n: u64 = 600851475143;

    let mut factors: Vec<u64> = vec![];

    let max: u64 = (n as f64).sqrt().ceil() as u64;
     
    while n % 2 == 0 {
        n/= 2;
        factors.push(2);
    }

    for num in (1..((max as f64)/2.0) as u64).map(|x| x*2+1) {
        while n % num == 0 {
            n /= num;
            factors.push(num);
        }
    }

    factors[factors.len()-1].to_string()
}

