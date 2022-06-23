//! A Rust library for parsing, serializing, and operating on hex colors.
//!
//! The [`HexColor`] contains the 3 RGB components and an optional alpha component.
//!
//! Suppoted parsing of 3, 6, 4 and 8 character hex codes. When converting to a string only the longer 6 and 8 character codes are produced.
//!
//! 3 and 6 character codes set [alpha] to `None`.
//! Same for converting to string: if [alpha] is `None` then a 6 character code is produced.
//! You can also make a color with no alpha by using [`HexColor::rgb()`] method.
//!
//! [alpha]: HexColor::a
//!
//! ```
//! # use hex_color::HexColor;
//! # use hex_color::ParseHexColorError;
//! # fn main() -> Result<(), ParseHexColorError> {
//! assert_eq!(HexColor::rgb(0x11, 0x22, 0x33), "#112233".parse::<HexColor>()?);
//! # Ok(())
//! # }
//! ```
//!
//! # Alpha component
//!
//! For 4 and 8 character codes alpha is set to `Some(value)`.
//! When converting to string such color will produce an 8 character code.
//! You can construct a color with alpha by using [`HexColor::rgba()`] The last argument in `rgba()` always sets
//! alpha to `Some(a)`.
//!
//! ```
//! # use hex_color::HexColor;
//! # use hex_color::ParseHexColorError;
//! # fn main() -> Result<(), ParseHexColorError> {
//! assert_eq!("#000f".parse::<HexColor>()?, HexColor::rgba(0, 0, 0, 255));
//! # Ok(())
//! # }
//! ```
//!
//! # Scalar scaling and adding.
//!
//! You can add, substract, multiply and divide a color. This applies the same operation to all components.
//!
//! ```
//! # use hex_color::HexColor;
//! assert_eq!(
//!     HexColor::rgb(0, 2, 7) + 3,
//!     HexColor::rgb(3, 5, 10),
//! );
//! assert_eq!(
//!     HexColor::rgba(0, 2, 7, 6) + 3,
//!     HexColor::rgba(3, 5, 10, 9),
//! );
//! ```
//!
//! As you can see if there is no alpha then it stays as `None`.
//!
//! # Operatons with 2 colors
//!
//! Corresponding components are added/multiplied/etc together. Following is a table explaining how alpha is calculated:
//!
//!
//! ```
//! # use hex_color::HexColor;
//! let blue = HexColor::rgba(0, 5, 0xff, 0x11);
//! let red = HexColor::rgba(0xff, 5, 0, 0xee);
//! let purple_green = HexColor::rgba(0xff, 10, 0xff, 0xff);
//!
//! assert_eq!(blue + red, purple_green);
//! ```

#![doc(html_root_url = "https://docs.rs/hex_color/1.0.0")]
#![warn(missing_docs)]

mod ops;

#[cfg(feature = "serde")]
mod serde;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use lazy_static::lazy_static;

use regex::Regex;

/// An RGB(A) color represented in hexadecimal.
///
/// # Examples
///
/// Create a hex color in a variety of ways:
///
/// *Note*: The parsing of hex colors is somewhat lenient: it does not need to
/// be prefixed with `#`, is trimmed before being parsed, is case insensitive,
/// and supports shorthand 3 and 4 character representations as well as the more
/// common 6 and 8.
///
/// ```
/// use hex_color::HexColor;
/// # use hex_color::ParseHexColorError;
///
/// # fn main() -> Result<(), ParseHexColorError> {
/// let black = HexColor { r: 0, g: 0, b: 0, a: None};
/// let gray = HexColor::new(127, 127, 127);
/// let white: HexColor = "#FFFFFF".parse()?;
/// # Ok(())
/// # }
/// ```
///
/// Operate on hex colors:
///
/// ```
/// use hex_color::HexColor;
/// # use hex_color::ParseHexColorError;
///
/// # fn main() -> Result<(), ParseHexColorError> {
/// let red: HexColor = "#F00".parse()?;
/// let blue: HexColor = "#00f".parse()?;
///
/// // You can add and subtract colors to make a new color after the operation
/// // is performed on each component, clamped to the range [0, 255]:
/// let purple = red + blue;
/// assert_eq!(purple, "f0f".parse::<HexColor>()?);
///
/// // You can scale a color's components through division and multiplication
/// // with a scalar value, clamped to the range [0, 255]:
/// let dark_purple = purple / 2;
/// assert_eq!(dark_purple, "#7F007F".parse::<HexColor>()?);
///
/// // You can add or subtract a scalar value from each component, clamped to
/// // the range [0, 255]:
/// let black = purple - 255;
/// assert_eq!(black, "000".parse::<HexColor>()?);
/// # Ok(())
/// # }
/// ```
///
/// ## Transparency calculations
///
/// | Left    | Right   | Result                                  |
/// |---------|---------|-----------------------------------------|
/// | `Some`  | `Some`  | Calculated the same way as other fields |
/// | _(any)_ | `None`  | Same as left's alpha (could be `None`)  |
/// | `None`  | _(any)_ | `None`                                  |
///
/// Convert a hex color to a string:
///
/// ```
/// use hex_color::HexColor;
///
/// // no alpha
/// let gray = HexColor { r: 127, g: 127, b: 127, a: None };
/// assert_eq!(gray.to_string(), "#7F7F7F");
///
/// // with alpha
/// let gray = HexColor { r: 127, g: 127, b: 127, a: Some(255) };
/// assert_eq!(gray.to_string(), "#7F7F7FFF");
/// ```
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HexColor {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
    /// Optional alpha component used in 4 and 8 character hex colors.
    /// If set to `None` then `to_string()` will not add any opacity.
    /// When parsing something like `"#fff"` This field will be set to `None` since there is no information about the alpha.
    /// `"#ffff"` however will set `a` to `Some(255)`.
    ///
    /// ```
    /// use hex_color::HexColor;
    /// # use hex_color::ParseHexColorError;
    ///
    /// # fn main() -> Result<(), ParseHexColorError> {
    /// let color = HexColor::rgba(1,2,3,4);
    /// let repr = String::from("#01020304");
    ///
    /// assert_eq!(repr.parse::<HexColor>()?, color);
    /// assert_eq!(repr, color.to_string());
    /// # Ok(())
    /// # }
    pub a: Option<u8>,
}

impl HexColor {
    /// Since the addition of alpha this function is deprecated.
    /// It's left in for backwards compatibility, but may be removed or changed in the future.
    /// Use [`rgb()`] or [`rgba()`] instead.
    /// Currently `new()` acts the same as [`rgb()`].
    ///
    /// [`rgb()`]: HexColor::rgb()
    /// [`rgba()`]: HexColor::rgba()
    #[deprecated]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r, g, b)
    }

    /// Creates a new hex color only with red, green and blue components.
    /// Alpha is set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_color::HexColor;
    ///
    /// let red = HexColor::rgb(255, 0, 0);
    /// assert_eq!(red.a, None);
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: None }
    }

    /// Creates a new color with red, green, blue and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_color::HexColor;
    ///
    /// let red = HexColor::rgba(255, 0, 0, 255);
    /// assert_eq!(red.a, Some(255));
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: Some(a),
        }
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.a {
            None => write!(f, "#{:02X?}{:02X?}{:02X?}", self.r, self.g, self.b),
            Some(a) => write!(
                f,
                "#{:02X?}{:02X?}{:02X?}{:02X?}",
                self.r, self.g, self.b, a
            ),
        }
    }
}

lazy_static! {
    static ref HEX_COLOR: Regex = Regex::new(r"(?i-u)^#?(?P<la>[[:xdigit:]]{8})|(?P<l>[[:xdigit:]]{6})|(?P<sa>[[:xdigit:]]{4})|(?P<s>[[:xdigit:]]{3})$").unwrap();
    static ref LA_COLOR: Regex = Regex::new(r"^([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})$").unwrap();
    static ref L_COLOR: Regex =  Regex::new(r"^([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})$").unwrap();
    static ref SA_COLOR: Regex = Regex::new(r"^([[:xdigit:]]{1})([[:xdigit:]]{1})([[:xdigit:]]{1})([[:xdigit:]]{1})$").unwrap();
    static ref S_COLOR: Regex =  Regex::new(r"^([[:xdigit:]]{1})([[:xdigit:]]{1})([[:xdigit:]]{1})$").unwrap();

}

impl FromStr for HexColor {
    type Err = ParseHexColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = Err(ParseHexColorError(s.to_owned()));

        let c = match HEX_COLOR.captures(s.trim()) {
            Some(c) => c,
            None => return err,
        };

        let which = |c: char| -> usize {
            match c {
                'r' => 1,
                'g' => 2,
                'b' => 3,
                'a' => 4,
                _ => unreachable!("Never call with {}", c),
            }
        };

        let get_long = |c: &regex::Captures, component: char| -> Result<u8, ParseHexColorError> {
            Ok(<u8>::from_str_radix(
                c.get(which(component))
                    .ok_or(ParseHexColorError(s.to_string()))?
                    .as_str(),
                16,
            )
            .or(Err(ParseHexColorError(s.to_string())))?)
        };

        let get_short = |c: &regex::Captures, component: char| -> Result<u8, ParseHexColorError> {
            Ok(<u8>::from_str_radix(
                &c.get(which(component))
                    .ok_or(ParseHexColorError(s.to_string()))?
                    .as_str()
                    .repeat(2),
                16,
            )
            .or(Err(ParseHexColorError(s.to_string())))?)
        };

        if let Some(c) = c.name("la") {
            let c = LA_COLOR
                .captures(c.as_str())
                .ok_or(ParseHexColorError(s.to_string()))?;
            return Ok(Self {
                r: get_long(&c, 'r')?,
                g: get_long(&c, 'g')?,
                b: get_long(&c, 'b')?,
                a: Some(get_long(&c, 'a')?),
            });
        }

        if let Some(c) = c.name("l") {
            let c = L_COLOR
                .captures(c.as_str())
                .ok_or(ParseHexColorError(s.to_string()))?;
            return Ok(Self {
                r: get_long(&c, 'r')?,
                g: get_long(&c, 'g')?,
                b: get_long(&c, 'b')?,
                a: None,
            });
        }

        if let Some(c) = c.name("sa") {
            let c = SA_COLOR
                .captures(c.as_str())
                .ok_or(ParseHexColorError(s.to_string()))?;
            return Ok(Self {
                r: get_short(&c, 'r')?,
                g: get_short(&c, 'g')?,
                b: get_short(&c, 'b')?,
                a: Some(get_short(&c, 'a')?),
            });
        }

        if let Some(c) = c.name("s") {
            let c = S_COLOR
                .captures(c.as_str())
                .ok_or(ParseHexColorError(s.to_string()))?;
            return Ok(Self {
                r: get_short(&c, 'r')?,
                g: get_short(&c, 'g')?,
                b: get_short(&c, 'b')?,
                a: None,
            });
        }

        err
    }
}

/// An error which can be returned when parsing a hex color.
///
/// This error is used as the error type for the [`FromStr`] implementation for
/// [`HexColor`]. The error contains a value that was not parsed.
///
/// ```
/// # use hex_color::{HexColor, ParseHexColorError};
/// assert_eq!("#GHIJKL".parse::<HexColor>(), ParseHexColorError("#GHIJKL"));
/// ```
///
/// # Panics
///
/// [`FromStr`] never panics.
#[derive(Debug)]
pub struct ParseHexColorError(pub String);

impl Display for ParseHexColorError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} could not be parsed as a hex color", self.0)
    }
}

impl Error for ParseHexColorError {}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test_case;
    use test_case::test_case;

    #[cfg(test)]
    #[test_case("fff", "s"; "short no alpha")]
    #[test_case("ffff", "sa"; "short with alpha")]
    #[test_case("ffFFff", "l"; "long no alpha")]
    #[test_case("ffFFffFF", "la"; "long with alpha")]
    fn regex_categorize_color(s: &str, n: &str) {
        HEX_COLOR
            .captures(s)
            .expect("Should be captured")
            .name(n)
            .expect(&format!(
                "hex = {}, len = {}; Captured as an incorrect group (expected {})",
                s,
                s.len(),
                n
            ));
    }

    #[test_case(0x00, 0x00, 0x00, "#000000"; "Black")]
    #[test_case(0x13, 0x69, 0x46, "#136946"; "Should parse correctly")]
    #[test_case(0xff, 0xff, 0xff, "#ffffff"; "White")]
    fn to_and_from_string_no_alpha(r: u8, g: u8, b: u8, s: &str) -> Result<(), ParseHexColorError> {
        let hex_color = HexColor::rgb(r, g, b);

        assert_eq!(s.parse::<HexColor>()?, hex_color);
        assert_eq!(hex_color.to_string().to_lowercase(), s.to_lowercase());

        Ok(())
    }

    #[test_case(HexColor{r: 0xff, g: 0x11, b: 0x55, a: None}, "#f15"; "Short no alpha")]
    fn supported_formats(color: HexColor, repr: &str) -> Result<(), ParseHexColorError> {
        assert_eq!(color, repr.parse::<HexColor>()?);
        Ok(())
    }

    #[test]
    fn any_format() -> Result<(), ParseHexColorError> {
        let mut bitset = BitSet::new();
        loop {
            let mut color = HexColor::rgb(255, 255, 255);
            let mut repr: String = "fff".into();
            // Bitset is used here to implement an iteration over all possible types of hex codes:
            // e.g. capitalised, with and without the "#", etc.
            //
            // Adding more checks requires just adding another `if bitset.next()` and running some code instead of
            // performing multiple steps like adding another for loop, not forgetting to permute a value
            // or making a list of all possible values like `["#fff", "#FFF", ...]`.

            if bitset.next() {
                repr += "f";
                color.a = Some(255);
            }

            if bitset.next() {
                repr = repr.to_uppercase();
            }

            if bitset.next() {
                repr = repr.repeat(2);
            }

            if bitset.next() {
                repr = "#".to_owned() + &repr;
            }

            if bitset.end() {
                break;
            }

            assert_eq!(repr.parse::<HexColor>()?, color, "from value: {}", repr);
        }
        return Ok(());

        /// Used in this test to go over every possible combination of hex code qualities, e.g. capitalised, with "#", has alpha.
        struct BitSet {
            bits: i128,
            ptr: usize,
        }
        impl BitSet {
            /// Creates a new bitset.
            fn new() -> Self {
                Self { bits: 0, ptr: 0 }
            }

            /// Yields the next bit from a bitset as bool.
            fn next(&mut self) -> bool {
                let bit = (self.bits >> self.ptr) & 1;
                self.ptr += 1;
                bit != 0
            }

            /// Finishes a bitset iteration. Increments bitset value and resets its pointer.
            /// Used to indicate that all properties have been read and now we should check if
            /// there is an "overflow".
            ///
            /// # Return value
            ///
            /// Returns true if internal pointer points to a 1.
            fn end(&mut self) -> bool {
                let end = self.next();
                self.ptr = 0;
                self.bits += 1;
                end
            }
        }
    }
}
