use std::{
    mem,
    ops::{Add, Div, Mul, Sub},
};

/// It can represent any number up to 8.pow(MAX::usize).
/// For instance,
/// ```rust
/// let val_u128 = Big::<32>::zero();
/// ```
/// is an equivalent of u256.
/// See tests for examples.
#[derive(Clone, Copy, PartialEq)]
pub struct Big<const BASE_8: usize> {
    bytes: [u8; BASE_8],
}

impl<const BASE_8: usize> Big<BASE_8> {
    pub fn zero() -> Self {
        Self { bytes: [0; BASE_8] }
    }

    pub fn one() -> Self {
        let mut bytes = [0; BASE_8];
        bytes[0] = 1;

        Self { bytes }
    }

    pub fn from(bytes: [u8; BASE_8]) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> [u8; BASE_8] {
        self.bytes
    }
}

impl<const BASE_8: usize> Add<Big<BASE_8>> for Big<BASE_8> {
    type Output = Self;

    fn add(mut self, rhs: Big<BASE_8>) -> Self::Output {
        let mut cmp = 0;

        for i in 0..BASE_8 {
            let (a, b) = (self.bytes[i], rhs.bytes[i]);
            let (val, mov) = a.overflowing_add(b);
            let c = if mov {
                mem::replace(&mut cmp, 1)
            } else {
                mem::take(&mut cmp)
            };

            self.bytes[i] = match val.overflowing_add(c) {
                (val, true) => {
                    cmp += 1;
                    val
                }
                (val, false) => val,
            };
        }

        if cmp > 0 {
            panic!("add with overflow");
        }

        self
    }
}

impl<const BASE_8: usize> Sub<Big<BASE_8>> for Big<BASE_8> {
    type Output = Self;

    fn sub(mut self, rhs: Big<BASE_8>) -> Self::Output {
        let mut cmp = 0;

        for i in 0..BASE_8 {
            let a = match self.bytes[i].overflowing_sub(cmp) {
                (val, true) => {
                    cmp = 1;
                    val
                }
                (val, false) => {
                    cmp = 0;
                    val
                }
            };
            let b = rhs.bytes[i];

            self.bytes[i] = match a.overflowing_sub(b) {
                (val, true) => {
                    cmp += 1;
                    val
                }
                (val, false) => val,
            };
        }

        if cmp > 0 {
            panic!("sub with overflow");
        }

        self
    }
}

// naive implementation
impl<const BASE_8: usize> Mul<Big<BASE_8>> for Big<BASE_8> {
    type Output = Self;

    fn mul(self, mut rhs: Big<BASE_8>) -> Self::Output {
        let zero = Big::zero();
        let one = Big::one();
        let mut res = zero;

        loop {
            if rhs == zero {
                break res;
            }
            rhs = rhs - one;
            res = res + self;
        }
    }
}

impl<const BASE_8: usize> Div<Big<BASE_8>> for Big<BASE_8> {
    type Output = Self;

    fn div(self, rhs: Big<BASE_8>) -> Self::Output {
        assert!(rhs.bytes.into_iter().any(|b| b > 0), "division by zero");

        let one = Big::one();

        let mut q = Big::zero();
        let mut r = self;

        loop {
            if r < rhs {
                break q;
            }
            q = q + one;
            r = r - rhs;
        }
    }
}

impl<const BASE_8: usize> PartialOrd for Big<BASE_8> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::Equal;

        for i in (0..BASE_8).rev() {
            match self.bytes[i].cmp(&other.bytes[i]) {
                Equal => continue,
                no_equal => return Some(no_equal),
            }
        }

        Some(Equal)
    }
}

#[cfg(test)]
mod l121 {
    use super::*;

    #[test]
    fn add_as_u64() {
        let a_u64 = 1e10 as u64;
        let b_u64 = 1e8 as u64;

        let mut bytes = a_u64.to_be_bytes();
        bytes.reverse();
        let a = Big::from(bytes);

        bytes = b_u64.to_be_bytes();
        bytes.reverse();
        let b = Big::from(bytes);

        let c_64 = a_u64 + b_u64;
        bytes = c_64.to_be_bytes();
        bytes.reverse();

        assert_eq!((a + b).bytes(), bytes);
    }

    #[test]
    fn sub_as_u64() {
        let a_u64 = 1e10 as u64;
        let b_u64 = 1e8 as u64;

        let mut bytes = a_u64.to_be_bytes();
        bytes.reverse();
        let a = Big::from(bytes);

        bytes = b_u64.to_be_bytes();
        bytes.reverse();
        let b = Big::from(bytes);

        let c_64 = a_u64 - b_u64;
        bytes = c_64.to_be_bytes();
        bytes.reverse();
        assert_eq!((a - b).bytes(), bytes);
    }

    #[test]
    fn div_as_u64() {
        let a_u64 = 1e10 as u64;
        let b_u64 = 1e8 as u64;

        let mut bytes = a_u64.to_be_bytes();
        bytes.reverse();
        let a = Big::from(bytes);

        bytes = b_u64.to_be_bytes();
        bytes.reverse();
        let b = Big::from(bytes);

        let mut c_64 = a_u64 / b_u64;
        bytes = c_64.to_be_bytes();
        bytes.reverse();
        assert_eq!((a / b).bytes(), bytes);

        c_64 = b_u64 / a_u64;
        bytes = c_64.to_be_bytes();
        bytes.reverse();
        assert_eq!((b / a).bytes(), bytes);
    }

    #[test]
    fn mul_as_u64() {
        let a_u64 = 1e8 as u64;
        let b_u64 = 1e3 as u64;

        let mut bytes = a_u64.to_be_bytes();
        bytes.reverse();
        let a = Big::from(bytes);

        bytes = b_u64.to_be_bytes();
        bytes.reverse();
        let b = Big::from(bytes);

        let c_64 = a_u64 * b_u64;
        bytes = c_64.to_be_bytes();
        bytes.reverse();
        assert_eq!((a * b).bytes(), bytes);
    }
}
