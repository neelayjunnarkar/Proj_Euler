
pub fn run() -> String {

    let mut max = 0;
    let mut nums: Vec<u64> = vec![];
    for init_num in (1..1_000_000) {
        let len = CollatzSeq::new(init_num).take_while(|n| *n != 1).collect::<Vec<u64>>().len()+2;
        if len  > max { max = len; nums.push(init_num); }
    }

    nums[nums.len()-1].to_string()
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
        self.n = if self.n % 2 == 0 { self.n/2 } else { 3*self.n + 1 };
        Some(self.n)
    }
}