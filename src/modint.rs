use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Type to hold a value in ModInt.
pub type ModValue = u64;

/// Used to pass a modulus to ModInt.
///
/// # Examples
///
/// ```
/// use tklib::modint;
///
/// #[derive(Debug, Copy, Clone)]
/// struct Mod13 {}
/// impl modint::ModTrait for Mod13 {
///     fn modulus() -> modint::ModValue {
///         13
///     }
/// }
/// type ModInt13 = modint::ModInt<Mod13>;
///
/// assert_eq!(ModInt13::new(5), ModInt13::new(2) - ModInt13::new(10));
/// ```
pub trait ModTrait: Debug + Copy + Clone {
    fn modulus() -> ModValue;
}

/// The static F_p integer type.
///
/// # Examples
///
/// ```
/// use tklib::modint;
///
/// type Mint = modint::ModInt998244353;
/// assert_eq!(Mint::new(1), Mint::new(998244352) + Mint::new(2));
/// assert_eq!(Mint::new(6), Mint::new(2) * Mint::new(3));
/// ```
#[derive(Debug, Copy, Clone)]
pub struct ModInt<Mod: ModTrait> {
    value: ModValue,
    _marker: std::marker::PhantomData<fn() -> Mod>,
}

impl<Mod: ModTrait> ModInt<Mod> {
    fn new_unchecked(value: ModValue) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    /// Constructs a new ModInt.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::modint;
    ///
    /// type Mint = modint::ModInt998244353;
    ///
    /// assert_eq!(Mint::new(2), Mint::new(998244354) + Mint::new(1));
    /// ```
    pub fn new(value: ModValue) -> Self {
        Self::new_unchecked(if value < Mod::modulus() {
            value
        } else {
            value % Mod::modulus()
        })
    }

    /// Returns the raw value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::modint;
    ///
    /// type Mint = modint::ModInt998244353;
    ///
    /// assert_eq!(3, Mint::new(3).value());
    /// ```
    pub fn value(self) -> ModValue {
        self.value
    }

    /// Takes the inverse of self, using the extended Euclidean algorithm.
    /// The greatest common divisor of `self.value()`
    /// and the modulus is required to be 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::modint;
    ///
    /// type Mint = modint::ModInt1000000007;
    ///
    /// assert_eq!(Mint::new(500000004), Mint::new(2).inv());
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn inv(self) -> Self {
        use std::mem::swap;

        assert_ne!(
            self,
            Self::new_unchecked(0),
            "Attempted to take the inverse of 0"
        );

        let mut a = self.value();
        let mut b = Mod::modulus();
        let mut x = Self::new_unchecked(1);
        let mut y = Self::new_unchecked(0);

        while a != 0 {
            let q = b / a;
            b -= a * q;
            y -= x * Self::new(q);
            swap(&mut a, &mut b);
            swap(&mut x, &mut y);
        }

        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(x, Self::new_unchecked(0));

        y
    }

    /// Raises self to the power of exp, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// ```
    /// use tklib::modint;
    ///
    /// type Mint = modint::ModInt998244353;
    ///
    /// assert_eq!(Mint::new(4), Mint::new(2).pow(2));
    /// assert_eq!(
    ///     Mint::new(926495343),
    ///     Mint::new(3).pow(5_000_000_000_000_000)
    /// );
    /// ```
    pub fn pow(self, mut exp: u64) -> Self {
        let mut base = self;
        let mut acc = Self::new_unchecked(1);

        while exp > 0 {
            if (exp & 1) == 1 {
                acc *= base;
            }
            exp >>= 1;
            base = base * base;
        }

        acc
    }
}

//
// Ops
//
#[allow(clippy::suspicious_arithmetic_impl)]
impl<Mod: ModTrait> Add for ModInt<Mod> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let out = self.value + rhs.value;
        Self::new_unchecked(if out < Mod::modulus() {
            out
        } else {
            out - Mod::modulus()
        })
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<Mod: ModTrait> Div for ModInt<Mod> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl<Mod: ModTrait> Mul for ModInt<Mod> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.value * rhs.value)
    }
}

impl<Mod: ModTrait> Neg for ModInt<Mod> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_unchecked(if self.value == 0 {
            0
        } else {
            Mod::modulus() - self.value
        })
    }
}

impl<Mod: ModTrait> Sub for ModInt<Mod> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new_unchecked(if self.value < rhs.value {
            (Mod::modulus() + self.value) - rhs.value
        } else {
            self.value - rhs.value
        })
    }
}

macro_rules! op_assign_impl {
    ($($trait: ident, $op_assign: ident, $op: ident)+) => {
        $(
            impl<Mod: ModTrait> $trait for ModInt<Mod> {
                fn $op_assign(&mut self, rhs: Self) {
                    *self = self.$op(rhs);
                }
            }
        )+
    };
}

op_assign_impl! {
    AddAssign, add_assign, add
    DivAssign, div_assign, div
    MulAssign, mul_assign, mul
    SubAssign, sub_assign, sub
}

//
// comparison
//
impl<Mod: ModTrait> PartialEq for ModInt<Mod> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Mod: ModTrait> Eq for ModInt<Mod> {}

//
//  Frequently used modulus
//
mod detail {
    #[derive(Debug, Copy, Clone)]
    pub struct Mod1000000007 {}

    impl super::ModTrait for Mod1000000007 {
        fn modulus() -> super::ModValue {
            1_000_000_007
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Mod998244353 {}

    impl super::ModTrait for Mod998244353 {
        fn modulus() -> super::ModValue {
            998_244_353
        }
    }
}
pub type ModInt1000000007 = ModInt<detail::Mod1000000007>;
pub type ModInt998244353 = ModInt<detail::Mod998244353>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(2), Mint::new(1) + Mint::new(1));

        let mut n = Mint::new(2);
        n += Mint::new(998244352);
        assert_eq!(n, Mint::new(1));
    }

    #[test]
    fn div() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(3), Mint::new(6) / Mint::new(2));

        let mut n = Mint::new(3);
        n /= Mint::new(499122177);
        assert_eq!(n, Mint::new(6));
    }

    #[test]
    #[should_panic]
    fn inv() {
        #[derive(Debug, Copy, Clone)]
        struct Mod6 {}

        impl ModTrait for Mod6 {
            fn modulus() -> ModValue {
                6
            }
        }

        type Mint = ModInt<Mod6>;

        let _ = Mint::new(3).inv();
    }

    #[test]
    fn mul() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(6), Mint::new(2) * Mint::new(3));

        let mut n = Mint::new(998244352);
        n *= Mint::new(998244352);
        assert_eq!(n, Mint::new(1));
    }

    #[test]
    fn neg() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(1), -Mint::new(998244352));
        assert_eq!(Mint::new(998244352), -Mint::new(1));
    }

    #[test]
    fn sub() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(1), Mint::new(2) - Mint::new(1));

        let mut n = Mint::new(1);
        n -= Mint::new(2);
        assert_eq!(n, Mint::new(998244352));
    }

    #[test]
    fn pow() {
        type Mint = ModInt998244353;
        assert_eq!(Mint::new(4), Mint::new(2).pow(2));
        assert_eq!(
            Mint::new(926495343),
            Mint::new(3).pow(5_000_000_000_000_000)
        );
    }
}
