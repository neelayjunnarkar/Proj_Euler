
pub fn run() -> String {
    TriangularNum::new()
	.skip_while(|n| num_divisors(*n) <= 500)
	.next()
	.unwrap()
	.to_string()
}


struct TriangularNum {
    curr: u64,
    c: u32
}

impl TriangularNum {
    fn new() -> TriangularNum {
        TriangularNum{curr: 0, c: 1}
    }
}

impl Iterator for TriangularNum {
    type Item = u64;
    
    fn next(&mut self) -> Option<<TriangularNum as Iterator>::Item> {
        self.curr += self.c as u64;
        self.c += 1;
        Some(self.curr)
    }
    
}

fn num_divisors(mut n: u64) -> u32 {
   let mut num_divisors: u32 = 0;
    if (n as f64).sqrt() as u64 == ((n as f64).sqrt() as u64) * ((n as f64).sqrt() as u64) {
        for num in 1 as u64..( (n as f64).sqrt().ceil() as u64) {
            if n % num == 0 {
                num_divisors += 1;
            }
        }

        num_divisors /= 2;
    } else {
        for num in 1 as u64..( (n as f64).sqrt().ceil() as u64) {
            if n % num == 0 {
                num_divisors += 1;
            }
        }
    }
    num_divisors*2
}