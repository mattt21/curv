use num_integer::Integer;
use rand::{rngs::OsRng, RngCore};

use super::traits::{BitManipulation, Converter, One, Samplable, Sign, Zero};
use super::BigInt;

impl Samplable for BigInt {
    fn sample_below(upper: &Self) -> Self {
        assert!(*upper > Self::zero());

        let bits = upper.bit_length();
        loop {
            let n = Self::sample(bits);
            if n < *upper {
                return n;
            }
        }
    }

    fn sample_range(lower: &Self, upper: &Self) -> Self {
        assert!(upper > lower);
        lower + Self::sample_below(&(upper - lower))
    }

    fn strict_sample_range(lower: &Self, upper: &Self) -> Self {
        assert!(upper > lower);
        loop {
            let n = lower + Self::sample_below(&(upper - lower));
            if n > *lower && n < *upper {
                return n;
            }
        }
    }

    fn sample(bit_size: usize) -> Self {
        if bit_size == 0 {
            return BigInt::zero();
        }
        let mut rng = OsRng::new().unwrap();
        let bytes = bit_size.div_ceil(&8);
        let mut buf: Vec<u8> = vec![0; bytes];
        rng.fill_bytes(&mut buf);
        BigInt::from_bytes(Sign::Positive, &*buf) >> (bytes * 8 - bit_size)
    }

    fn strict_sample(bit_size: usize) -> Self {
        if bit_size == 0 {
            return BigInt::zero();
        }
        let lower = BigInt::one() << (bit_size - 1);
        let upper = BigInt::one() << bit_size;
        BigInt::sample_range(&lower, &upper)
    }
}
