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

impl Add for HexColor {
    type Output = HexColor;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        HexColor::new(
            u8::saturating_add(self.r, other.r),
            u8::saturating_add(self.g, other.g),
            u8::saturating_add(self.b, other.b),
        )
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
                HexColor::new(
                    (self.r as $t + other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t + other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t + other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
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
        HexColor::new(
            u8::saturating_sub(self.r, other.r),
            u8::saturating_sub(self.g, other.g),
            u8::saturating_sub(self.b, other.b),
        )
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
                HexColor::new(
                    (self.r as $t - other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t - other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t - other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
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
                HexColor::new(
                    (self.r as $t * other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t * other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t * other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
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
                HexColor::new(
                    (self.r as $t / other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t / other).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t / other).clamp(u8::MIN as $t, u8::MAX as $t) as u8
                )
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

    const ZERO: HexColor = HexColor::new(0, 0, 0);
    const ONE: HexColor = HexColor::new(1, 1, 1);
    const TWO: HexColor = HexColor::new(2, 2, 2);
    const MAX: HexColor = HexColor::new(255, 255, 255);

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
