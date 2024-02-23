use image::{math::Rect, DynamicImage, GenericImageView};

use super::{pixel_is_consistent, Algorithm};

#[deprecated]
pub(crate) struct Original;

//Traverse the image as a 1D array until the first non-white pixel is found. (x1, y1)
//March straight down from that pixel until the first white pixel is found. (y2)
//March to the right from first pixel until the first white pixel is found. (x2)
//The cropped image can be found using the found coordinates. (x1, y1, x2 - x1, y2 - y1)
//This algorithm is deprecated because it only handles images where the content is aligned to 90 degrees.
//All other images will yield invalid cropped sub-images.
#[allow(deprecated)]
impl Algorithm for Original {
    fn find_photo(img: &DynamicImage) -> Option<Rect> {
        //Find the first non-white pixel in the image via 1D array (0..img.width * img.height) traversal.
        //This will provide 2/4 coordinates required to crop the image.
        let (x1, y1) = img.pixels().into_iter().find(|(_, _, pix)| {
            !pixel_is_consistent(pix)
        }).map(|(x, y, _)| (x, y)).expect("No starting pixel was found within the criteria.");

        //From the x1 value found before, march straight down in the image until the first white pixel is found.
        //This pixel provides the y2 value.
        let mut y2 = img.height();
        for dy in y1..img.height() {
            let pix = &img.get_pixel(x1, dy);
            if pixel_is_consistent(pix) {
                y2 = dy;
                break;
            }
        }

        //Finally, from the y1 value from before, march right until a white pixel is found.
        //This will give us the final coordinate, x2.
        let mut x2 = img.width();
        for dx in x1..img.width() {
            let pix = &img.get_pixel(dx, y1);
            if pixel_is_consistent(pix) {
                x2 = dx;
                break;
            }
        }

        Some(Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1
        })
    }
}