//Longest Collatz sequence for starting number under 1 million

pub fn run() -> String {
    (1..1_000_000)
        .map(|x| (CollatzSeq::new(x).count()+2, x))
        .max()
        .unwrap()
        .1
        .to_string()
}

struct CollatzSeq {
  n: u64  
}

impl CollatzSeq {
    fn new(initial_n: u64) -> CollatzSeq { CollatzSeq{n: initial_n} }
} 

impl Iterator for CollatzSeq {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.n == 1 {
            None
        } else {
            self.n = if self.n % 2 == 0 { self.n/2 } else { 3*self.n + 1 };
            Some(self.n)
        }
    }
}