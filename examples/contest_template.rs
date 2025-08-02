use std::io;
use turbo_input::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());

    let t: usize = scan.token(); // number of test cases

    for _ in 0..t {
        solve(&mut scan);
    }
}

fn solve(scan: &mut Scanner<std::io::StdinLock>) {
    let n: usize = scan.token();
    let arr: Vec<i64> = scan.vec(n);

    // Example: find maximum element
    let max_val = arr.iter().max().unwrap_or(&0);
    println!("{}", max_val);
}
