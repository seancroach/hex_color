//! A Rust library for parsing, serializing, and operating on hex colors.

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

/// An RGB color represented in hexadecimal.
///
/// # Examples
///
/// Create a hex color in a variety of ways:
///
/// *Note*: The parsing of hex colors is somewhat lenient: it does not need to
/// be prefixed with `#`, is trimmed before being parsed, is case insensitive,
/// and supports shorthand three-character representations as well as the more
/// common six.
///
/// ```
/// use hex_color::HexColor;
/// # use hex_color::ParseHexColorError;
///
/// # fn main() -> Result<(), ParseHexColorError> {
/// let black = HexColor { r: 0, g: 0, b: 0 };
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
/// Convert a hex color to a string:
///
/// ```
/// use hex_color::HexColor;
///
/// let gray = HexColor { r: 127, g: 127, b: 127 };
///
/// assert_eq!(gray.to_string(), "#7F7F7F");
/// ```
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HexColor {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
}

impl HexColor {
    /// Creates a new hex color with the given red, green, and blue components.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_color::HexColor;
    ///
    /// let red = HexColor::new(255, 0, 0);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        HexColor { r, g, b }
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{:02X?}{:02X?}{:02X?}", self.r, self.g, self.b)
    }
}

lazy_static! {
    static ref HEX_COLOR_3_REGEX: Regex =
        Regex::new("(?i)^#?(?P<r>[\\da-f])(?P<g>[\\da-f])(?P<b>[\\da-f])$").unwrap();
    static ref HEX_COLOR_6_REGEX: Regex =
        Regex::new("(?i)^#?(?P<r>[\\da-f]{2})(?P<g>[\\da-f]{2})(?P<b>[\\da-f]{2})$").unwrap();
}

impl FromStr for HexColor {
    type Err = ParseHexColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        // TODO: Find a better way of doing this?
        let (r, g, b) = if let Some(c) = HEX_COLOR_3_REGEX.captures(s) {
            (
                c.name("r").unwrap().as_str().repeat(2),
                c.name("g").unwrap().as_str().repeat(2),
                c.name("b").unwrap().as_str().repeat(2),
            )
        } else if let Some(c) = HEX_COLOR_6_REGEX.captures(s) {
            (
                c.name("r").unwrap().as_str().to_string(),
                c.name("g").unwrap().as_str().to_string(),
                c.name("b").unwrap().as_str().to_string(),
            )
        } else {
            return Err(ParseHexColorError(s.to_string()));
        };
        Ok(HexColor {
            r: u8::from_str_radix(&r, 16).unwrap(),
            g: u8::from_str_radix(&g, 16).unwrap(),
            b: u8::from_str_radix(&b, 16).unwrap(),
        })
    }
}

/// An error which can be returned when parsing a hex color.
///
/// This error is used as the error type for the [`FromStr`] implementation for
/// [`HexColor`].
///
/// # Examples
///
/// The following code would panic from a `ParseHexColorError` as `"#GHIJKL"` is
/// not a valid hex color:
///
/// ```should_panic
/// use hex_color::HexColor;
/// # use hex_color::ParseHexColorError;
///
/// # fn main() -> Result<(), ParseHexColorError> {
/// let invalid_color = "#GHIJKL".parse::<HexColor>()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ParseHexColorError(String);

impl Display for ParseHexColorError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} could not be parsed as a hex color", self.0)
    }
}

impl Error for ParseHexColorError {}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: HexColor = HexColor::new(0, 0, 0);
    const GRAY: HexColor = HexColor::new(127, 127, 127);
    const WHITE: HexColor = HexColor::new(255, 255, 255);

    const BLACK_STR: &'static str = "#000000";
    const GRAY_STR: &'static str = "#7F7F7F";
    const WHITE_STR: &'static str = "#FFFFFF";

    #[test]
    fn from_str_accuracy() -> Result<(), ParseHexColorError> {
        assert_eq!(BLACK_STR.parse::<HexColor>()?, BLACK);
        assert_eq!(GRAY_STR.parse::<HexColor>()?, GRAY);
        assert_eq!(WHITE_STR.parse::<HexColor>()?, WHITE);

        Ok(())
    }

    #[test]
    fn from_str() -> Result<(), ParseHexColorError> {
        assert_eq!("fff".parse::<HexColor>()?, WHITE);
        assert_eq!("FFF".parse::<HexColor>()?, WHITE);
        assert_eq!("#fff".parse::<HexColor>()?, WHITE);
        assert_eq!("#FFF".parse::<HexColor>()?, WHITE);
        assert_eq!("ffffff".parse::<HexColor>()?, WHITE);
        assert_eq!("FFFFFF".parse::<HexColor>()?, WHITE);
        assert_eq!("#ffffff".parse::<HexColor>()?, WHITE);
        assert_eq!("#FFFFFF".parse::<HexColor>()?, WHITE);

        Ok(())
    }

    #[test]
    fn to_string() -> Result<(), ParseHexColorError> {
        assert_eq!(BLACK.to_string(), BLACK_STR);
        assert_eq!(GRAY.to_string(), GRAY_STR);
        assert_eq!(WHITE.to_string(), WHITE_STR);

        Ok(())
    }
}
