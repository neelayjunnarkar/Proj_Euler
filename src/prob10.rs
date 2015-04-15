//Summation of Primes

pub fn run() -> String {

    ((1..1_000_000u64)
        .map(|n| n*2+1)
        .filter(|&n| -> bool {
            for k in (1..((n as f64).sqrt().ceil() as u64 + 1)/2).map(|n| n*2+1) { 
                if n % k == 0 { return false } 
            } 
            return true
        })
        .filter(|&n| n <= 2_000_000)
        .fold(0, |acc, item| acc+item)+2)
	.to_string()
  
}