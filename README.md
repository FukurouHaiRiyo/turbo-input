
# turbo-input

A fast and efficient scanner library for competitive programming in Rust. This library provides convenient methods for reading various types of input commonly encountered in programming contests.


## Features

- Fast input reading with internal buffering
- Type-safe parsing with automatic type inference
- Common data structures like vectors, matrices, and graphs
- Zero dependencies - uses only the Rust standard library
- Comprehensive documentation with examples
- Thoroughly tested with unit tests


## Installation

Add this to your Cargo.toml:

```bash
    [dependencies]
    turbo-input = "0.1.0"
```
    
## Quick start

```rust
    use std::io;
    use turbo_input::Scanner;

    fn main() {
        let stdin = io::stdin();
        let mut scan = Scanner::new(stdin.lock());
        
        // Read different types
        let n: i32 = scan.token();
        let name: String = scan.string();
        let numbers: Vec<i32> = scan.vec(n as usize);
        
        println!("Read {} numbers: {:?}", n, numbers);
}
```
## Usage/Examples

### Basic Token Readin
```rust
use turbo_input::Scanner;

let input = "42 3.14 hello";
let mut scanner = Scanner::new(input.as_bytes());

let number: i32 = scanner.token();      // 42
let float: f64 = scanner.token();       // 3.14
let text: String = scanner.string();    // "hello"
```

### Reading vectors
```rust
let input = "5\n1 2 3 4 5";
let mut scanner = Scanner::new(input.as_bytes());

let n: usize = scanner.token();
let numbers: Vec<i32> = scanner.vec(n);  // [1, 2, 3, 4, 5]
```

### Reading matrices
```rust
let input = "1 2 3\n4 5 6\n7 8 9";
let mut scanner = Scanner::new(input.as_bytes());

let matrix: Vec<Vec<i32>> = scanner.matrix(3, 3); // [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
```

### Reading character arrays
```rust
let input = "hello";
let mut scanner = Scanner::new(input.as_bytes());

let chars: Vec<char> = scanner.chars();  // ['h', 'e', 'l', 'l', 'o']
```

### Reading graphs 
```rust
let input = "1 2\n2 3\n1 3";  // 3 edges
let mut scanner = Scanner::new(input.as_bytes());

// Read undirected graph with 3 vertices and 3 edges
let graph = scanner.graph(3, 3, false);
// graph[1] = [2, 3], graph[2] = [1, 3], graph[3] = [2, 1]
```



## API Reference

#### Scanner<R>

The main scanner struct that wraps any type implementing BufRead.
Methods:

- new(reader: R) -> Self - Creates a new scanner
- token<T>() -> T - Reads and parses the next token
- vec<T>(n: usize) -> Vec<T> - Reads n tokens into a vector
- matrix<T>(rows: usize, cols: usize) -> Vec<Vec<T>> - Reads a matrix
- chars() -> Vec<char> - Reads next token as character vector
- string() -> String - Reads next token as string
- graph(n: usize, m: usize, directed: bool) -> Vec<Vec<usize>> - Reads a graph

All parsing methods support any type that implements FromStr, including:

- Integers: i32, i64, u32, u64, usize, etc.
- Floating point: f32, f64
- Strings: String
And more!



## Performance

This scanner is designed for competitive programming where fast I/O is crucial. It:

- Buffers input internally to minimize system calls
- Uses efficient string parsing
- Minimizes memory allocations where possible
## Typical Competitive Programming Usage

```rust
use std::io;
use turbo_input::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scan = Scanner::new(stdin.lock());
    
    let t: usize = scan.token();  // number of test cases
    
    for _ in 0..t {
        let n: usize = scan.token();
        let arr: Vec<i64> = scan.vec(n);
        
        // Solve the problem...
        let result = solve(&arr);
        println!("{}", result);
    }
}

fn solve(arr: &[i64]) -> i64 {
    // Your solution here
    arr.iter().sum()
}
```
## Error handling

Currently, the scanner will panic on:

- I/O errors (failed reads)
- Parse errors (invalid format for requested type)

This is intentional for competitive programming where you want fast failure on invalid input rather than error handling overhead.
## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.


## License

This project is licensed under
- [MIT](https://choosealicense.com/licenses/mit/) license

## Important Note

This project was made so I can learn Rust and to have a better understanding on howuser input works in Rust, any suggestions/advice and forks are more than welcome.

## Changelog

### 0.1.0

- Initial release
- Basic scanner functionality
- Support for tokens, vectors, matrices, chars, strings, and graphs

### 0.1.0

- Bug fixes
