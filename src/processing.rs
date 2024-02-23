use std::{ffi::OsStr, path::{Path, PathBuf}};

use image::{io::Reader as ImgR, math::Rect, GenericImageView};

use crate::algs::Algorithm;

//Processes an image and saves the cropped output to a new image with
//a suffixed file name in the same directory it's located in.
pub(crate) fn process<Alg: Algorithm>(path: PathBuf) -> Option<PathBuf> {
    //Load an image from disk at the given path
    let Some(img) = ImgR::open(&path).ok().and_then(|img| img.decode().ok()) else {
        eprintln!("Failed to load and decode image.");
        return None;
    };

    //Attempt to find the minimum sub-image required to retain all image data while cropping out
    //extra white borders using the provided algorithm implementation.
    let Some(Rect { x, y, width, height }) = <Alg>::find_photo(&img) else {
        return None;
    };

    if x == 0 && width == img.width() && y == 0 && height == img.height() {
        println!("Skipping already cropped image.");
        return None;
    }

    //Retrieve a sub-image view using the cropped viewport found.
    let photo = img.view(x, y, width, height);

    let path = get_new_path(&path);

    //Save the newly cropped version in the same location with a suffixed name.
    photo.to_image().save(&path).expect("Failed to save img");

    Some(path)
}

//Given a path, determine the new filename of a cropped image based off the original.
//"C:\some image.png" -> "C:\Cropped some image.png"
fn get_new_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let buf = path.as_ref().to_path_buf();

    let ext = buf.extension().map(OsStr::to_string_lossy).expect("Failed to get extension");
    let fname = buf.file_stem().map(OsStr::to_string_lossy).expect("Failed to get file stem");
    let parent_path = buf.parent().expect("Failed to get parent");
    
    let mut child_buf = parent_path.to_path_buf();
    child_buf.push(format!("Cropped {fname}.{ext}"));
    child_buf
}