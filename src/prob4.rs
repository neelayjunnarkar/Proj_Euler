//Largest Palindrome Product
use std::collections::BinaryHeap;
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
    let mut palindromes: BinaryHeap<i32> = BinaryHeap::new();
    for a in Range::new(999, 100, -1) {
        for b in Range::new(999, 100, -1) {
            let d = a*b;
            let c = (a*b).to_string();
            if c == c.chars().rev().collect::<String>() {
                palindromes.push(d);
            }
        }   
    }
    palindromes.peek().unwrap().to_string()
}
