use std::{ffi::OsStr, path::{Path, PathBuf}};

use image::{io::Reader as ImgR, math::Rect, GenericImageView};

use crate::algs::Algorithm;

pub(crate) fn process<A: Algorithm>(path: PathBuf) {
    let Some(img) = ImgR::open(&path).ok().and_then(|img| img.decode().ok()) else {
        eprintln!("Failed to load and decode image.");
        return;
    };

    let Some(Rect { x, y, width, height }) = <A>::find_photo(&img) else {
        println!("\"{}\" is already cropped.", path.file_name().map(OsStr::to_string_lossy).unwrap());
        return;
    };

    let photo = img.view(x, y, width, height);
    photo.to_image().save(get_new_path(&path)).expect("Failed to save img");
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