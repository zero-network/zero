use crate::arithmetic::limbs::{add, double, mul, neg, square, sub};
use crate::coordinate::Projective;
use crate::domain::field::field_operation;
use crate::error::Error;
use crate::interface::coordinate::Coordinate;
use core::{
    cmp::Ordering,
    fmt::{Binary, Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Neg, Sub},
    ops::{AddAssign, MulAssign, SubAssign},
};
use parity_scale_codec::{Decode, Encode};
use rand_core::RngCore;

pub(crate) const MODULUS: &[u64; 4] = &[
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: &[u64; 4] = &[
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: &[u64; 4] = &[
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

pub(crate) const INV: u64 = 0x1ba3a358ef788ef9;

const S: u32 = 1;

const ROOT_OF_UNITY: &[u64; 4] = &[
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
];

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fr(pub(crate) [u64; 4]);

field_operation!(Fr, MODULUS);

impl Fr {
    pub fn from_hex(hex: &str) -> Result<Fr, Error> {
        let max_len = 64;
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let length = hex.len();
        if length > max_len {
            return Err(Error::HexStringTooLong);
        }
        let hex_bytes = hex.as_bytes();

        let mut hex: [[u8; 16]; 4] = [[0; 16]; 4];
        for i in 0..max_len {
            hex[i / 16][i % 16] = if i >= length {
                0
            } else {
                match hex_bytes[length - i - 1] {
                    48..=57 => hex_bytes[length - i - 1] - 48,
                    65..=70 => hex_bytes[length - i - 1] - 55,
                    97..=102 => hex_bytes[length - i - 1] - 87,
                    _ => return Err(Error::HexStringInvalid),
                }
            };
        }
        let mut limbs: [u64; 4] = [0; 4];
        for i in 0..hex.len() {
            limbs[i] = Fr::bytes_to_u64(&hex[i]).unwrap();
        }
        Ok(Fr(limbs))
    }

    fn to_bytes(&self) -> [u8; 64] {
        let mut bytes: [u8; 64] = [0; 64];
        let mut index = 15;
        for i in 0..self.0.len() {
            let mut number = self.0[i];
            for n in 0..16 {
                let quotient = number as u128 / 16_u128.pow(15 - n as u32);
                bytes[index - n] = quotient as u8;
                number = (number as u128 % 16_u128.pow(15 - n as u32)) as u64;
            }
            index += 16;
        }
        bytes
    }

    fn to_bits(&self) -> [u8; 256] {
        let mut index = 0;
        let mut bits: [u8; 256] = [0; 256];
        for mut x in self.0 {
            for _ in 0..64 {
                bits[index] = (x & 1) as u8;
                x = x >> 1;
                index += 1;
            }
        }
        bits
    }

    fn from_u512(limbs: [u64; 8]) -> Self {
        let a = mul(&[limbs[0], limbs[1], limbs[2], limbs[3]], R2, &MODULUS);
        let b = mul(&[limbs[4], limbs[5], limbs[6], limbs[7]], R3, &MODULUS);
        let c = add(&a, &b, &MODULUS);
        Fr(c)
    }

    fn bytes_to_u64(bytes: &[u8; 16]) -> Result<u64, Error> {
        let mut res: u64 = 0;
        for i in 0..bytes.len() {
            res += match bytes[i] {
                0..=15 => 16u64.pow(i as u32) * bytes[i] as u64,
                _ => return Err(Error::BytesInvalid),
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::coordinate::Coordinate;

    #[test]
    fn test_is_zero() {
        let fr = Fr([0, 0, 0, 0]);
        assert!(fr.is_zero());
        let fr = Fr([0, 0, 0, 1]);
        assert!(!fr.is_zero());
    }

    #[test]
    fn test_fmt_and_to_bin() {
        let fr = Fr([
            0xd0970e5ed6f72cb7,
            0xa6682093ccc81082,
            0x06673b0101343b00,
            0x0e7db4ea6533afa9,
        ]);
        libc_print::libc_println!("{}", fr);
        libc_print::libc_println!("{:b}", fr);
    }

    #[test]
    fn test_binary_method() {
        let fr = Fr([3, 3, 3, 3]);
        libc_print::libc_println!("{}", fr);
        let base = Projective::one();
        libc_print::libc_println!("{:?}", base);
        let res = fr.binary_method(&base);
        libc_print::libc_println!("{:?}", res);
    }

    #[test]
    fn test_from_hex() {
        let a = Fr::from_hex("0x64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab")
            .unwrap();
        assert_eq!(
            a,
            Fr([
                0xb9feffffffffaaab,
                0x1eabfffeb153ffff,
                0x6730d2a0f6b0f624,
                0x64774b84f38512bf,
            ])
        )
    }

    #[test]
    fn test_cmp() {
        let a = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad82")
            .unwrap();
        let b = Fr::from_hex("0x6fa7bab5fb3a644af160302de3badc0958601b445c9713d2b7cdba213809ad83")
            .unwrap();

        assert_eq!(a <= a, true);
        assert_eq!(a >= a, true);
        assert_eq!(a == a, true);
        assert_eq!(a < b, true);
        assert_eq!(a > b, false);
        assert_eq!(a != b, true);
        assert_eq!(a == b, false);
    }
}
