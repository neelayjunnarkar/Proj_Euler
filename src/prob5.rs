struct Range {
    curr: i32,
    next: i32,
    step: i32,
    end: i32,
}
impl Range {
    fn new(start: i32, end: i32, step: i32) -> Range {
        Range{curr: start, end: end, step: step, next: start+step}
    }
}
impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.curr = self.next;
        self.next = self.next + self.step;
        if self.step > 0 && self.curr > self.end {
            None
        } else if self.step < 0 && self.curr < self.end {
            None
        } else {
            Some(self.curr)
        }
    }
}

pub fn run() -> String {
	let mut c = 1;
	'infin: loop {
		for i in Range::new(20, 1, -1){
			if c*20 % i != 0 {
				c += 1;
				continue 'infin;
			}
		}
		break;		
	} 

	(c*20).to_string()
}

