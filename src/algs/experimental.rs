use image::{math::Rect, DynamicImage, GenericImageView, Rgba};

use crate::algs::pixel_is;

use super::Algorithm;

pub(crate) struct Experimental;

impl Algorithm for Experimental {
    fn find_photo(img: &DynamicImage) -> Option<Rect> {
        let is_white_pixel = |pix: &Rgba<u8>| pixel_is(pix, (255, 255, 255), 10);

        let mut x1 = None;
        let mut y1 = None;
        let mut x2 = None;
        let mut y2 = None;

        'find_left_border: for x in 0..img.width() {
            for y in 0..img.height() {
                let pix = &img.get_pixel(x, y);
                if !is_white_pixel(pix) {
                    x1 = Some(x);
                    break 'find_left_border;
                }
            }
        }

        'find_right_border: for x in (0..img.width()).rev() {
            for y in 0..img.height() {
                let pix = &img.get_pixel(x, y);
                if !is_white_pixel(pix) {
                    x2 = Some(x);
                    break 'find_right_border;
                }
            }
        }

        'find_top_border: for y in 0..img.height() {
            for x in 0..img.width() {
                let pix = &img.get_pixel(x, y);
                if !is_white_pixel(pix) {
                    y1 = Some(y);
                    break 'find_top_border;
                }
            }
        }

        'find_bottom_border: for y in (0..img.height()).rev() {
            for x in 0..img.width() {
                let pix = &img.get_pixel(x, y);
                if !is_white_pixel(pix) {
                    y2 = Some(y);
                    break 'find_bottom_border;
                }
            }
        }

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