use crate::HexColor;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
impl Distribution<HexColor> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HexColor {
        HexColor::rgb(rng.gen(), rng.gen(), rng.gen())
    }
}