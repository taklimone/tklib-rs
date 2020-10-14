//! Fenwick Tree.

/// Fenwick Tree. 1-indexed.
///
/// # Examples
///
/// ```
/// use tklib::data_structures::fenwick::Fenwick;
///
/// let a = [1, 2, 3, 4, 5];
/// let mut fw = Fenwick::from_slice(&a);
///
/// assert_eq!(15, fw.sum(5));
/// assert_eq!(5, fw.sum(3) - fw.sum(1));
///
/// fw.add(3, 10);
/// assert_eq!(16, fw.sum(3));
/// assert_eq!(13, fw.sum(3) - fw.sum(2));
/// ```
pub struct Fenwick {
    table: Vec<i64>,
}

impl Fenwick {
    /// Constructs a new Fenwick Tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::data_structures::fenwick::Fenwick;
    ///
    /// let mut fw = Fenwick::new();
    /// ```
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { table: vec![0] }
    }

    /// Constructs a new Fenwick Tree from a slice.
    ///
    /// # Examples
    /// ```
    /// use tklib::data_structures::fenwick::Fenwick;
    ///
    /// let a = [1, 2, 3];
    /// let fw = Fenwick::from_slice(&a);
    /// ```
    pub fn from_slice(src: &[i64]) -> Self {
        let n = src.len();

        let mut table = vec![0; n + 1];
        table[1..].copy_from_slice(src);

        (1..n)
            .map(|i| (i, i + lsb(i)))
            .filter(|&(_, j)| j <= n)
            .for_each(|(i, j)| table[j] += table[i]);

        Self { table }
    }

    /// Pushes a new element x.
    ///
    /// # Examples
    /// ```
    /// use tklib::data_structures::fenwick::Fenwick;
    ///
    /// let mut fw = Fenwick::new();
    /// fw.push(1);
    /// ```
    pub fn push(&mut self, x: i64) {
        let n = self.table.len();
        let k = lsb(n);

        self.table.push(
            std::iter::successors(Some(1), |&i| Some(i * 2))
                .take_while(|&i| i != k)
                .map(|i| self.table[n - i])
                .fold(x, std::ops::Add::add),
        )
    }

    /// Sums up the elements in [1, i].
    ///
    /// # Examples
    /// ```
    /// use tklib::data_structures::fenwick::Fenwick;
    ///
    /// let mut fw = Fenwick::new();
    /// fw.push(1);
    /// fw.push(2);
    /// assert_eq!(1, fw.sum(1));
    /// assert_eq!(3, fw.sum(2));
    /// ```
    pub fn sum(&self, i: usize) -> i64 {
        std::iter::successors(Some(i), |&i| Some(i - lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.table[i])
            .sum()
    }

    /// Adds x onto the i-th element.
    ///
    /// # Examples
    /// ```
    /// use tklib::data_structures::fenwick::Fenwick;
    ///
    /// let mut fw = Fenwick::new();
    /// fw.push(1);
    /// fw.push(2);
    /// assert_eq!(3, fw.sum(2));
    ///
    /// fw.add(2, 3);
    /// assert_eq!(6, fw.sum(2));
    /// ```
    pub fn add(&mut self, i: usize, x: i64) {
        let n = self.table.len();
        std::iter::successors(Some(i), |&i| Some(i + lsb(i)))
            .take_while(|&i| i < n)
            .for_each(|i| self.table[i] += x);
    }
}

fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_slice() {
        // add a random test
        todo!();
    }
}
