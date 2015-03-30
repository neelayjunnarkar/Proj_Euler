
struct Fibonacci {
    prev: i32,
    curr: i32
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci{prev: 0, curr: 1}
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let temp = self.prev;
        self.prev = self.curr;
        self.curr = self.prev+temp;
        Some(self.curr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}


fn main() {
    println!("{}", Fibonacci::new().
        take_while(|&f| f < 4_000_000).
        filter(|&f| f%2 == 0).
        fold(0, |acc, item| acc+item));
}