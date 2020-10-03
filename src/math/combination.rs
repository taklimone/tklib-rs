/// Calculates combinations mod p.
///
/// # Examples
///
/// ```
/// use tklib::math::combination::Combination;
///
/// let comb = Combination::new(5_000_000, 1_000_000_007);
///
/// assert_eq!(1, comb.com(2, 0));
/// assert_eq!(1, comb.com(3, 3));
/// assert_eq!(6, comb.com(4, 2));
/// assert_eq!(828_782_236, comb.com(5_000_000, 2_500_000));
/// ```
pub struct Combination {
    fac: Vec<u64>,
    facinv: Vec<u64>,
    modulus: u64,
}

impl Combination {
    /// Construction.
    /// Consumes O(maximum + log(modulus)) time and O(maximum) space.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::math::combination::Combination;
    ///
    /// let comb = Combination::new(5_000_000, 1_000_000_007);
    /// ```
    pub fn new(maximum: u64, modulus: u64) -> Self {
        assert!(maximum < modulus);

        let fac: Vec<u64> = std::iter::once(1)
            .chain((1..=maximum).scan(1, |state, x| {
                *state *= x;
                if *state >= modulus {
                    *state %= modulus;
                }
                Some(*state)
            }))
            .collect();

        let fac_maximum_inv = Self::inv(fac[maximum as usize], modulus);

        let mut facinv: Vec<u64> = std::iter::once(fac_maximum_inv)
            .chain((1..=maximum).rev().scan(fac_maximum_inv, |state, x| {
                *state *= x;
                if *state >= modulus {
                    *state %= modulus;
                }
                Some(*state)
            }))
            .collect();
        facinv.reverse();

        Self {
            fac,
            facinv,
            modulus,
        }
    }

    /// Returns nCm.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::math::combination::Combination;
    ///
    /// let comb = Combination::new(5_000_000, 1_000_000_007);
    ///
    /// assert_eq!(6, comb.com(4, 2));
    /// ```
    pub fn com(&self, n: u64, m: u64) -> u64 {
        assert!(n >= m);
        (self.fac(n) * self.facinv(m) % self.modulus) * self.facinv(n - m) % self.modulus
    }

    /// Returns n!.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tklib::math::combination::Combination;
    ///
    /// let comb = Combination::new(5_000_000, 1_000_000_007);
    ///
    /// assert_eq!(120, comb.fac(5));
    /// ```
    pub fn fac(&self, n: u64) -> u64 {
        let n = n as usize;
        assert!(n < self.fac.len());
        self.fac[n]
    }

    /// Returns the inverse of n!.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tklib::math::combination::Combination;
    ///
    /// let comb = Combination::new(5_000_000, 1_000_000_007);
    ///
    /// assert_eq!(808_333_339, comb.facinv(5));
    /// ```
    pub fn facinv(&self, n: u64) -> u64 {
        let n = n as usize;
        assert!(n < self.fac.len());
        self.facinv[n]
    }

    #[allow(clippy::many_single_char_names)]
    fn inv(a: u64, p: u64) -> u64 {
        let mut acc = 1;
        let mut base = a;
        let mut exp = p - 2;

        while exp > 0 {
            if (exp & 1) == 1 {
                acc *= base;
                if acc >= p {
                    acc %= p
                }
            }
            exp >>= 1;
            base = base * base;
            if base >= p {
                base %= p
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let comb = Combination::new(5_000_000, 1_000_000_007);

        assert_eq!(1, comb.com(2, 0));
        assert_eq!(1, comb.com(3, 3));
        assert_eq!(6, comb.com(4, 2));
        assert_eq!(828_782_236, comb.com(5_000_000, 2_500_000));
    }
}
