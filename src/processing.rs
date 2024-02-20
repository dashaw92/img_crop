use std::{ffi::OsStr, path::{Path, PathBuf}};

use image::{io::Reader as ImgR, math::Rect, DynamicImage, GenericImageView, Rgba};

pub fn process(path: PathBuf) {
    let Ok(Ok(img)) = ImgR::open(&path).map(|img| img.decode()) else {
        eprintln!("Failed to load and decode image.");
        return;
    };

    let Some(Rect { x, y, width, height }) = find_photo(&img) else {
        println!("\"{}\" is already cropped.", path.file_name().map(OsStr::to_string_lossy).unwrap());
        return;
    };

    let photo = img.view(x, y, width, height);
    photo.to_image().save(get_new_path(&path)).expect("Failed to save img");
}

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

fn pixel_is(pix: &Rgba<u8>, color: (u8, u8, u8), epsilon: u8) -> bool {
    let [r, g, b, _] = pix.0;

    color.0.abs_diff(r) < epsilon 
    && color.1.abs_diff(g) < epsilon 
    && color.2.abs_diff(b) < epsilon
}

fn get_new_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let buf = path.as_ref().to_path_buf();

    let ext = buf.extension().map(OsStr::to_string_lossy).expect("Failed to get extension");
    let fname = buf.file_stem().map(OsStr::to_string_lossy).expect("Failed to get file stem");
    let path = buf.parent().expect("Failed to get parent");
    
    let mut child_buf = path.to_path_buf();
    child_buf.push(format!("{fname} - Cropped.{ext}"));
    child_buf
}