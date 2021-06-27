use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

use crate::HexColor;

struct HexColorVisitor;

impl<'de> Visitor<'de> for HexColorVisitor {
    type Value = HexColor;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "a hexadecimal color")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<HexColor, E> {
        HexColor::from_str(value).map_err(|e| de::Error::custom(e))
    }
}

impl Serialize for HexColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexColorVisitor)
    }
}
