
fn main() {

    let mut primes: Vec<u32> = vec![2];

    (1..55_250).map(|x|x*2+1)
        .filter(|&x| -> bool {
                for prime in &primes { 
                    if x % *prime == 0 && x != *prime { return false }
                    if *prime > (x as f64).sqrt() as u32 { break; }
                }
                primes.push(x);
                true
            })
        .collect::<Vec<u32>>();

    println!("{}", primes[10_000]);

}