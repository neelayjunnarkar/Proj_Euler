
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 { 
        println!("error: args should be in form: {} <max_number>", args[0]); 
        return; 
    }
    let max: u32 = std::num::from_str_radix(args[1].as_slice().trim(), 10).ok().expect("u32 expected");
    println!("max: {}", max);


    let res: u32 = (0..max).filter(|x| x%3 == 0 || x%5 == 0).fold(0, |acc, iter| acc+iter);
    println!("res: {}", res);

}