use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::HexColor;

#[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
impl Distribution<HexColor> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HexColor {
        let u = rng.next_u32();
        HexColor::rgb(
            (u & 0xff) as u8,
            ((u >> 8) & 0xff) as u8,
            ((u >> 16) & 0xff) as u8,
        )
    }
}
