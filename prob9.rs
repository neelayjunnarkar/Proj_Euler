//Pythagorean Triplet

fn main() {
    let mut a = 1;

    'outer: loop {
        for b in (a+1)..(1000+a) {
            if a*a + b*b == (1000-a-b)*(1000-a-b) {
                println!("a+b+c: {}", a+b+(1000-a-b));
                println!("abc: {}", a*b*(1000-a-b));
                break 'outer;
            }
        }
        a += 1;
    }
}