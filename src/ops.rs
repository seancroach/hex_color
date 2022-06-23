use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::HexColor;

macro_rules! forward_ref_binop {
    (impl $trait:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $trait<$u> for &'a $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, other)
            }
        }

        impl $trait<&$u> for $t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $trait<$u>>::Output {
                $trait::$method(self, *other)
            }
        }

        impl $trait<&$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $trait<$u>>::Output {
                $trait::$method(*self, *other)
            }
        }
    };
}

macro_rules! forward_ref_op_assign {
    (impl $trait:ident, $method:ident for $t:ty, $u:ty) => {
        impl $trait<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $trait::$method(self, *other);
            }
        }
    };
}

fn alpha_op(self_a: Option<u8>, other_a: Option<u8>, op: impl FnOnce(u8, u8) -> u8) -> Option<u8> {
    if let None = other_a {
        return self_a;
    }
    match self_a {
        Some(a) => Some(op(a, other_a.expect("Should never panic."))),
        None => None,
    }
}

impl Add for HexColor {
    type Output = HexColor;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        HexColor {
            r: u8::saturating_add(self.r, other.r),
            g: u8::saturating_add(self.g, other.g),
            b: u8::saturating_add(self.b, other.b),
            a: alpha_op(self.a, other.a, |a, o| u8::saturating_add(a, o)),
        }
    }
}

forward_ref_binop! { impl Add, add for HexColor, HexColor }

impl AddAssign for HexColor {
    #[inline]
    fn add_assign(&mut self, other: HexColor) {
        *self = *self + other;
    }
}

forward_ref_op_assign! { impl AddAssign, add_assign for HexColor, HexColor }

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add<$t> for HexColor {
            type Output = HexColor;

            #[inline]
            fn add(self, other: $t) -> Self::Output {
                let calc = |s: u8| (s as $t + other).clamp(u8::MIN as $t, u8::MAX as $t) as u8;
                HexColor{
                    r: calc(self.r),
                    g: calc(self.g),
                    b: calc(self.b),
                    a: match self.a {
                        Some(a) => Some(calc(a)),
                        None => None
                    },
                }
            }
        }

        impl Add<HexColor> for $t  {
            type Output = HexColor;

            #[inline]
            fn add(self, other: HexColor) -> Self::Output {
                other + self
            }
        }

        forward_ref_binop! { impl Add, add for HexColor, $t }
        forward_ref_binop! { impl Add, add for $t, HexColor }

        impl AddAssign<$t> for HexColor {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                *self = *self + other;
            }
        }

        forward_ref_op_assign! { impl AddAssign, add_assign for HexColor, $t }
    )*)
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Sub for HexColor {
    type Output = HexColor;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        HexColor {
            r: u8::saturating_sub(self.r, other.r),
            g: u8::saturating_sub(self.g, other.g),
            b: u8::saturating_sub(self.b, other.b),
            a: alpha_op(self.a, other.a, |a, o| u8::saturating_sub(a, o)),
        }
    }
}

forward_ref_binop! { impl Sub, sub for HexColor, HexColor }

impl SubAssign for HexColor {
    #[inline]
    fn sub_assign(&mut self, other: HexColor) {
        *self = *self - other;
    }
}

forward_ref_op_assign! { impl SubAssign, sub_assign for HexColor, HexColor }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub<$t> for HexColor {
            type Output = HexColor;

            #[inline]
            fn sub(self, other: $t) -> Self::Output {
                let calc = |s|(s as $t - other).clamp(u8::MIN as $t, u8::MAX as $t) as u8;
                HexColor{
                    r: calc(self.r),
                    g: calc(self.g),
                    b: calc(self.b),
                    a: match self.a {
                        Some(a) => Some(calc(a)),
                        None => None
                    }
                }
            }
        }

        impl Sub<HexColor> for $t  {
            type Output = HexColor;

            #[inline]
            fn sub(self, other: HexColor) -> Self::Output {
                other - self
            }
        }

        forward_ref_binop! { impl Sub, sub for HexColor, $t }
        forward_ref_binop! { impl Sub, sub for $t, HexColor }

        impl SubAssign<$t> for HexColor {
            #[inline]
            fn sub_assign(&mut self, other: $t) {
                *self = *self - other;
            }
        }

        forward_ref_op_assign! { impl SubAssign, sub_assign for HexColor, $t }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul<$t> for HexColor {
            type Output = HexColor;

            #[inline]
            fn mul(self, other: $t) -> Self::Output {
               let calc = |s|(s as $t * other).clamp(u8::MIN as $t, u8::MAX as $t) as u8;
                HexColor{
                    r: calc(self.r),
                    g: calc(self.g),
                    b: calc(self.b),
                    a: match self.a {
                        Some(a) => Some(calc(a)),
                        None => None
                    }
                }
            }
        }

        impl Mul<HexColor> for $t  {
            type Output = HexColor;

            #[inline]
            fn mul(self, other: HexColor) -> Self::Output {
                other * self
            }
        }

        forward_ref_binop! { impl Mul, mul for HexColor, $t }
        forward_ref_binop! { impl Mul, mul for $t, HexColor }

        impl MulAssign<$t> for HexColor {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                *self = *self * other;
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for HexColor, $t }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<$t> for HexColor {
            type Output = HexColor;

            #[inline]
            fn div(self, other: $t) -> Self::Output {
                let calc = |s|(s as $t / other).clamp(u8::MIN as $t, u8::MAX as $t) as u8;
                HexColor{
                    r: calc(self.r),
                    g: calc(self.g),
                    b: calc(self.b),
                    a: match self.a {
                        Some(a) => Some(calc(a)),
                        None => None
                    }
                }
            }
        }

        forward_ref_binop! { impl Div, div for HexColor, $t }

        impl DivAssign<$t> for HexColor {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                *self = *self / other;
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for HexColor, $t }
    )*)
}

div_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test_case;
    use test_case::test_case;

    const ZERO: HexColor = HexColor::rgb(0, 0, 0);
    const ONE: HexColor = HexColor::rgb(1, 1, 1);
    const TWO: HexColor = HexColor::rgb(2, 2, 2);
    const MAX: HexColor = HexColor::rgb(255, 255, 255);

    #[test_case(Some(2),   Some(3),   Some(2+3); "Calculate")]
    #[test_case(Some(100), None,      Some(100); "Same as left")]
    #[test_case(None,      None,      None;      "None")]
    #[test_case(None,      Some(100), None;      "Always None")]
    fn alha_op_no_panic(a: Option<u8>, b: Option<u8>, r: Option<u8>) {
        assert_eq!(alpha_op(a, b, |a, b| a + b), r);
    }

    #[test]
    fn add_hex() {
        assert_eq!(ONE + ONE, TWO);
        assert_eq!(ONE + &ONE, TWO);
        assert_eq!(&ONE + ONE, TWO);
        assert_eq!(&ONE + &ONE, TWO);
    }

    #[test]
    fn add_hex_scalar() {
        assert_eq!(ONE + 1, TWO);
        assert_eq!(1 + ONE, TWO);
        assert_eq!(ONE + &1, TWO);
        assert_eq!(&1 + ONE, TWO);
        assert_eq!(&ONE + 1, TWO);
        assert_eq!(&1 + ONE, TWO);
        assert_eq!(&ONE + &1, TWO);
        assert_eq!(&1 + &ONE, TWO);
    }

    #[test]
    fn add_hex_overflow() {
        assert_eq!(MAX + MAX, MAX);
    }

    #[test]
    fn add_hex_assign() {
        let mut value = ONE.clone();
        value += ONE;
        assert_eq!(value, TWO);

        let mut value = ONE.clone();
        value += &ONE;
        assert_eq!(value, TWO);
    }

    #[test]
    fn add_hex_scalar_assign() {
        let mut value = ONE.clone();
        value += 1;
        assert_eq!(value, TWO);

        let mut value = ONE.clone();
        value += &1;
        assert_eq!(value, TWO);
    }

    #[test]
    fn sub_hex() {
        assert_eq!(ONE - ONE, ZERO);
        assert_eq!(ONE - &ONE, ZERO);
        assert_eq!(&ONE - ONE, ZERO);
        assert_eq!(&ONE - ONE, ZERO);
    }

    #[test]
    fn sub_hex_scalar() {
        assert_eq!(ONE - 1, ZERO);
        assert_eq!(1 - ONE, ZERO);
        assert_eq!(ONE - &1, ZERO);
        assert_eq!(&1 - ONE, ZERO);
        assert_eq!(&ONE - 1, ZERO);
        assert_eq!(&1 - ONE, ZERO);
        assert_eq!(&ONE - &1, ZERO);
        assert_eq!(&1 - &ONE, ZERO);
    }

    #[test]
    fn sub_hex_underflow() {
        assert_eq!(ZERO - ONE, ZERO);
    }

    #[test]
    fn sub_hex_scalar_underflow() {
        assert_eq!(ZERO - 1, ZERO);
    }

    #[test]
    fn sub_hex_assign() {
        let mut value = ONE.clone();
        value -= ONE;
        assert_eq!(value, ZERO);

        let mut value = ONE.clone();
        value -= &ONE;
        assert_eq!(value, ZERO);
    }

    #[test]
    fn sub_hex_scalar_assign() {
        let mut value = ONE.clone();
        value -= 1;
        assert_eq!(value, ZERO);

        let mut value = ONE.clone();
        value -= &1;
        assert_eq!(value, ZERO);
    }

    #[test]
    fn mul_hex_scalar() {
        assert_eq!(ONE * 2, TWO);
        assert_eq!(2 * ONE, TWO);
        assert_eq!(ONE * &2, TWO);
        assert_eq!(&2 * ONE, TWO);
        assert_eq!(&ONE * 2, TWO);
        assert_eq!(&2 * ONE, TWO);
        assert_eq!(&ONE * &2, TWO);
        assert_eq!(&2 * &ONE, TWO);
    }

    #[test]
    fn mul_hex_scalar_assign() {
        let mut value = ONE.clone();
        value *= 2;
        assert_eq!(value, TWO);

        let mut value = ONE.clone();
        value *= &2;
        assert_eq!(value, TWO);
    }

    #[test]
    fn mul_hex_scalar_overflow() {
        assert_eq!(MAX * 2, MAX);
    }

    #[test]
    fn mul_hex_scalar_underflow() {
        assert_eq!(MAX * -1, ZERO);
    }

    #[test]
    fn div_hex_scalar() {
        assert_eq!(TWO / 2, ONE);
        assert_eq!(TWO / &2, ONE);
        assert_eq!(&TWO / 2, ONE);
        assert_eq!(&TWO / &2, ONE);
    }

    #[test]
    fn div_hex_scalar_assign() {
        let mut value = TWO.clone();
        value /= 2;
        assert_eq!(value, ONE);

        let mut value = TWO.clone();
        value /= &2;
        assert_eq!(value, ONE);
    }

    #[test]
    fn div_hex_scalar_overflow() {
        assert_eq!(MAX / 0.01, MAX);
    }

    #[test]
    fn div_hex_scalar_underflow() {
        assert_eq!(MAX / -1, ZERO);
    }
}
