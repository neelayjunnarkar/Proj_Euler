//Pythagorean Triplet

pub fn run() -> String {
    for a in 1..1000 {
        for b in 1..1000 {
            if a + b >= 1000 {
                continue;
            }
            if let Some(c) = c(a, b) {
                if a + b + c == 1000 {
                    return (a*b*c).to_string();
                }
            }
        }
    }
    "Problem 9: Error".to_string()
}

fn c(a: u32, b: u32) -> Option<u32> {
    let c: f64 = ((a.pow(2) + b.pow(2)) as f64).sqrt();
    if c == c.round() {
        Some(c as u32)
    } else {
        None
    }
}