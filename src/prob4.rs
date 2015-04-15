//Largest Palindrome Product

pub fn run() -> String {
    let mut x: u32 = 0;
    for num in (999..100).step_by(-1) {
        println!("num: {}", num);
        for a in (999..100).step_by(-1) {
            println!("a: {}", a);
            if is_palindrome(a.to_string().as_ref()) { 


                x = a;
            }
        }
    }
    x.to_string()
}

    
fn is_palindrome(string: &str) -> bool {
    println!("{}", string);
    let reversed = string.chars().rev().collect::<Vec<char>>();

    for i in (0..string.len()-1) {
        if string.char_at(i) != reversed[i]  { return false; }
    }
    true
}