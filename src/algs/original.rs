use image::{math::Rect, DynamicImage, GenericImageView};

use super::{pixel_is, Algorithm};

pub(crate) struct Original;

impl Algorithm for Original {
    fn find_photo(img: &DynamicImage) -> Option<Rect> {
        let (x1, y1) = img.pixels().into_iter().find(|(_, _, pix)| {
            !pixel_is(pix, (255, 255, 255), 10)
        }).map(|(x, y, _)| (x, y)).expect("No starting pixel was found within the criteria (step 1)");

        let mut y2 = img.height();
        for dy in y1..img.height() {
            let pix = &img.get_pixel(x1, dy);
            if pixel_is(pix, (255, 255, 255), 10) {
                y2 = dy;
                break;
            }
        }

        let mut x2 = img.width();
        for dx in x1..img.width() {
            let pix = &img.get_pixel(dx, y1);
            if pixel_is(pix, (255, 255, 255), 10) {
                x2 = dx;
                break;
            }
        }

        if x1 == 0 && x2 == img.width() && y1 == 0 && y2 == img.height() {
            return None
        }

        Some(Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1
        })
    }
}