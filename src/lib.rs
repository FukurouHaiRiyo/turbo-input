use std::io::BufRead;

/// A fast scanner for competitive programming
/// 
/// This scanner provides efficient methods for reading various types of input
/// commonly used in competitive programming contests. It buffers input internally
/// to minimize the number of system calls.
/// 
/// # Examples
/// 
/// ```
/// use std::io;
/// use competitive_scanner::Scanner;
/// 
/// let input = "42 3.14 hello\n1 2 3\n";
/// let mut scanner = Scanner::new(input.as_bytes());
/// 
/// let number: i32 = scanner.token();
/// let float: f64 = scanner.token();
/// let text: String = scanner.token();
/// 
/// assert_eq!(number, 42);
/// assert_eq!(float, 3.14);
/// assert_eq!(text, "hello");
/// 
/// let vec: Vec<i32> = scanner.vec(3);
/// assert_eq!(vec, vec![1, 2, 3]);
/// ```
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    /// Creates a new Scanner from any type that implements BufRead
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Any type implementing BufRead (e.g., stdin, file, string)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use competitive_scanner::Scanner;
    /// 
    /// // From stdin
    /// let stdin = io::stdin();
    /// let mut scanner = Scanner::new(stdin.lock());
    /// 
    /// // From string
    /// let input = "1 2 3";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    /// Reads the next token and parses it to the specified type
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type to parse the token into. Must implement FromStr.
    /// 
    /// # Panics
    /// 
    /// Panics if reading fails or if parsing fails.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// let input = "42 3.14 hello";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let number: i32 = scanner.token();
    /// let float: f64 = scanner.token();
    /// let text: String = scanner.token();
    /// 
    /// assert_eq!(number, 42);
    /// assert_eq!(float, 3.14);
    /// assert_eq!(text, "hello");
    /// ```
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed to parse token");
            }

            let mut line = String::new();
            self.reader
                .read_line(&mut line)
                .expect("Failed to read line");
            
            self.buffer = line
                .split_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }

    /// Reads n tokens and returns them as a vector
    /// 
    /// # Arguments
    /// 
    /// * `n` - Number of tokens to read
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// let input = "1 2 3 4 5";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let numbers: Vec<i32> = scanner.vec(5);
    /// assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.token()).collect()
    }

    /// Reads a matrix of tokens with specified dimensions
    /// 
    /// # Arguments
    /// 
    /// * `rows` - Number of rows in the matrix
    /// * `cols` - Number of columns in the matrix
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// let input = "1 2 3\n4 5 6";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let matrix: Vec<Vec<i32>> = scanner.matrix(2, 3);
    /// assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    pub fn matrix<T: std::str::FromStr>(&mut self, rows: usize, cols: usize) -> Vec<Vec<T>> {
        (0..rows).map(|_| self.vec(cols)).collect()
    }

    /// Reads the next token as a string and returns it as a vector of characters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// let input = "hello";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let chars: Vec<char> = scanner.chars();
    /// assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
    /// ```
    pub fn chars(&mut self) -> Vec<char> {
        self.token::<String>().chars().collect()
    }

    /// Reads the next token as a string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// let input = "hello world";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let word1: String = scanner.string();
    /// let word2: String = scanner.string();
    /// 
    /// assert_eq!(word1, "hello");
    /// assert_eq!(word2, "world");
    /// ```
    pub fn string(&mut self) -> String {
        self.token::<String>()
    }

    /// Reads a graph representation and returns an adjacency list
    /// 
    /// # Arguments
    /// 
    /// * `n` - Number of vertices (vertices are numbered from 1 to n)
    /// * `m` - Number of edges
    /// * `directed` - Whether the graph is directed or undirected
    /// 
    /// # Returns
    /// 
    /// A vector of size n+1 where index i contains the neighbors of vertex i.
    /// Index 0 is unused to allow 1-based vertex numbering.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use competitive_scanner::Scanner;
    /// 
    /// // Undirected graph: 1-2, 2-3
    /// let input = "1 2\n2 3";
    /// let mut scanner = Scanner::new(input.as_bytes());
    /// 
    /// let graph = scanner.graph(3, 2, false);
    /// // graph[1] = [2], graph[2] = [1, 3], graph[3] = [2]
    /// assert_eq!(graph[1], vec![2]);
    /// assert_eq!(graph[2], vec![1, 3]);
    /// assert_eq!(graph[3], vec![2]);
    /// ```
    pub fn graph(&mut self, n: usize, m: usize, directed: bool) -> Vec<Vec<usize>> {
        let mut adj = vec![vec![]; n + 1];
        for _ in 0..m {
            let u: usize = self.token();
            let v: usize = self.token();
            adj[u].push(v);
            if !directed {
                adj[v].push(u);
            }
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_parsing() {
        let input = "42 3.14 hello";
        let mut scanner = Scanner::new(input.as_bytes());

        let number: i32 = scanner.token();
        let float: f64 = scanner.token();
        let text: String = scanner.token();

        assert_eq!(number, 42);
        assert_eq!(float, 3.14);
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_vec() {
        let input = "1 2 3 4 5";
        let mut scanner = Scanner::new(input.as_bytes());

        let numbers: Vec<i32> = scanner.vec(5);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_matrix() {
        let input = "1 2 3\n4 5 6";
        let mut scanner = Scanner::new(input.as_bytes());

        let matrix: Vec<Vec<i32>> = scanner.matrix(2, 3);
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_chars() {
        let input = "hello";
        let mut scanner = Scanner::new(input.as_bytes());

        let chars: Vec<char> = scanner.chars();
        assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_string() {
        let input = "hello world";
        let mut scanner = Scanner::new(input.as_bytes());

        let word1: String = scanner.string();
        let word2: String = scanner.string();

        assert_eq!(word1, "hello");
        assert_eq!(word2, "world");
    }

    #[test]
    fn test_undirected_graph() {
        let input = "1 2\n2 3\n1 3";
        let mut scanner = Scanner::new(input.as_bytes());

        let graph = scanner.graph(3, 3, false);
        
        assert_eq!(graph[1], vec![2, 3]);
        assert_eq!(graph[2], vec![1, 3]);
        assert_eq!(graph[3], vec![2, 1]);
    }

    #[test]
    fn test_directed_graph() {
        let input = "1 2\n2 3";
        let mut scanner = Scanner::new(input.as_bytes());

        let graph = scanner.graph(3, 2, true);
        
        assert_eq!(graph[1], vec![2]);
        assert_eq!(graph[2], vec![3]);
        assert_eq!(graph[3], vec![]);
    }
}