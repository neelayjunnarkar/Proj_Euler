
use std::num::Float;

pub fn run() -> String {
    (((1..101).fold(0, |acc, iter| acc+iter) as f32).powi(2) as u32 -
        (1..101).map(|x| x*x).fold(0, |acc, iter| acc+iter))
	.to_string()
}