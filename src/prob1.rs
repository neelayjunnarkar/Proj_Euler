
pub fn run() -> String {
    (1..1000).filter(|&x|x % 3 == 0 || x % 5 == 0).fold(0, |acc, elem| acc+elem).to_string()
}