
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

fn num_divisors(n: u64) -> u32 {
    (1 as u64..((n as f64).sqrt().ceil() as u64))
        .filter(|x| n % x == 0)
        .count() as u32
        *2
}