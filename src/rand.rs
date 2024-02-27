use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::HexColor;

#[cfg_attr(doc_cfg, doc(cfg(feature = "rand")))]
impl Distribution<HexColor> for Standard {
    #[inline]
    fn sample<R>(&self, rng: &mut R) -> HexColor where R: ?Sized + Rng {
        let [r, g, b, _] = rng.next_u32().to_ne_bytes();
        HexColor::rgb(r, g, b)
    }
}
