use std::io;
use turbo_input::Scanner;

fn main() {
    println!("Enter some numbers separated by spaces:");
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let n: usize = scan.token();
    println!("Reading {} numbers:", n);

    let numbers: Vec<i32> = scan.vec(n);
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", numbers.iter().sum::<i32>());
}
