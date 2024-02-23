use std::convert::identity;

use image::{math::Rect, DynamicImage, GenericImageView};

use super::{pixel_is_consistent, Algorithm};

pub(crate) struct Experimental;

//This algorithm "marches" up all four borders of the image, stopping once a non-white pixel
//has been found. This determines the coordinate that can be cropped up to without losing image data.
//This algorithm properly handles all images, including those not aligned to 90 degrees, unlike the Original
//algorithm. When applied to those images, this algorithm will retain all white space required to avoid
//losing image data.
impl Algorithm for Experimental {
    fn find_photo(img: &DynamicImage) -> Option<Rect> {
        //Add one to a coordinate to ensure image data isn't cut-off.
        //Clamps to the provided max value to maintain validity of coordinates.
        let add_clamped = |max: u32| {
            move |coord: u32| (coord + 1).min(max)
        };

        //Find all four borders
        let x1 = horz_border(0..img.width(), &img, identity);
        let y1 = vert_border(0..img.height(), &img, identity);
        let x2 = horz_border((0..img.width()).rev(), &img, add_clamped(img.width()));
        let y2 = vert_border((0..img.height()).rev(), &img, add_clamped(img.height()));

        //Unwrap all four coordinates into a rectangle or yield nothing.
        match (x1, y1, x2, y2) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => Some(Rect {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
            }),
            _ => None
        }
    }
}

//Find a vertical border using the provided iterator before transforming it via the mapping function.
fn vert_border<F>(y_iter: impl Iterator<Item = u32>, img: &DynamicImage, mapping: F) -> Option<u32> 
where F: Fn(u32) -> u32 {
    for y in y_iter {
        for x in 0..img.width() {
            let pix = &img.get_pixel(x, y);
            if !pixel_is_consistent(pix) {
                return Some(mapping(y))
            }
        }
    }

    None
}

//Find a horizontal border using the provided iterator before transforming it via the mapping function.
fn horz_border<F>(x_iter: impl Iterator<Item = u32>, img: &DynamicImage, mapping: F) -> Option<u32> 
where F: Fn(u32) -> u32 {
    for x in x_iter {
        for y in 0..img.height() {
            let pix = &img.get_pixel(x, y);
            if !pixel_is_consistent(pix) {
                return Some(mapping(x));
            }
        }
    }

    None
}