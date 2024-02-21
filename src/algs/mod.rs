use image::{math::Rect, DynamicImage, Rgba};

pub(crate) mod original;
pub(crate) mod experimental;

pub(crate) trait Algorithm {
    fn find_photo(img: &DynamicImage) -> Option<Rect>;
}

pub(crate) fn pixel_is(pix: &Rgba<u8>, color: (u8, u8, u8), epsilon: u8) -> bool {
    let [r, g, b, _] = pix.0;

    color.0.abs_diff(r) < epsilon 
    && color.1.abs_diff(g) < epsilon 
    && color.2.abs_diff(b) < epsilon
}
