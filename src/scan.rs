use cargo_snippet::snippet;

#[snippet(doc_hidden, "scan")]
use std::io::prelude::*;

#[snippet(doc_hidden, "scan")]
use std::str::FromStr;

/// Wraps a reader. Reads tokens.
#[snippet(doc_hidden, "scan")]
pub struct Scanner<R: Read>(R);

#[snippet(doc_hidden, "scan")]
impl<R: Read> Scanner<R> {
    /// Builds a scanner out of a reader.
    /// # Examples
    ///
    /// ```
    /// use tklib::scan::Scanner;
    ///
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    /// ```
    pub fn new(reader: R) -> Self {
        Self(reader)
    }

    /// Reads a token.
    ///
    /// # Panics
    /// Panics if it fails to parse a token.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::scan::Scanner;
    ///
    /// let input = "4 3";
    /// let mut sc = Scanner::new(input.as_bytes());
    ///
    /// let (n, m): (u32, u32) = (sc.read(), sc.read());
    /// assert_eq!((n, m), (4, 3));
    /// ```
    pub fn read<T: FromStr>(&mut self) -> T {
        let token = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| (b as char).is_whitespace())
            .take_while(|&b| !(b as char).is_whitespace())
            .collect::<Vec<u8>>();

        unsafe { std::str::from_utf8_unchecked(token.as_slice()) }
            .parse()
            .ok()
            .expect("Can't parse it.")
    }

    /// Reads n tokens. Returns in Vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::scan::Scanner;
    ///
    /// let input = "\
    /// 5
    /// 1 2 3 4 5
    /// ";
    /// let mut sc = Scanner::new(input.as_bytes());
    ///
    /// let n: usize = sc.read();
    /// assert_eq!(n, 5);
    ///
    /// let v: Vec<i32> = sc.vec(n);
    /// assert_eq!(v, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }

    /// Reads a string. Returns in Vec\<char\>.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::scan::Scanner;
    ///
    /// let input = ".#..#";
    /// let mut sc = Scanner::new(input.as_bytes());
    ///
    /// let v = sc.chars();
    /// assert_eq!(v, vec!['.', '#', '.', '.', '#']);
    /// ```
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string() {
        let input = "AtCoder";
        let mut sc = Scanner::new(input.as_bytes());

        let s: String = sc.read();
        assert_eq!(s, "AtCoder");
    }

    #[test]
    fn read_number() {
        let input = "1 3\n";
        let mut sc = Scanner::new(input.as_bytes());

        let (n, m): (i32, i32) = (sc.read(), sc.read());
        assert_eq!(n, 1);
        assert_eq!(m, 3);
    }

    #[test]
    fn read_graph() {
        let input = "\
        4 3
        1 2
        1 3
        1 4
        ";
        let mut sc = Scanner::new(input.as_bytes());

        let (n, m): (u32, u32) = (sc.read(), sc.read());
        assert_eq!((n, m), (4, 3));

        let (from, to): (u32, u32) = (sc.read(), sc.read());
        assert_eq!((from, to), (1, 2));

        let (from, to): (u32, u32) = (sc.read(), sc.read());
        assert_eq!((from, to), (1, 3));

        let (from, to): (u32, u32) = (sc.read(), sc.read());
        assert_eq!((from, to), (1, 4));
    }

    #[test]
    fn read_vec() {
        let input = "\
        5
        1 2 3 4 5
        ";
        let mut sc = Scanner::new(input.as_bytes());

        let n: usize = sc.read();
        assert_eq!(n, 5);

        let v: Vec<i32> = sc.vec(n);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn read_chars() {
        let input = ".#..#";
        let mut sc = Scanner::new(input.as_bytes());

        let v = sc.chars();
        assert_eq!(v, vec!['.', '#', '.', '.', '#']);
    }
}
