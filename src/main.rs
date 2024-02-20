#![windows_subsystem = "windows"]

use std::{ffi::OsStr, fs::metadata, path::{Path, PathBuf}, str::FromStr};

mod processing;

fn main() {
    //Load the target file/directory from the argument, or default to the CWD
    let path = std::env::args().nth(1).unwrap_or(".".to_owned());
    let meta = metadata(&path).expect("Couldn't get metadata for target.");

    let targets = if meta.is_dir() {
        let arg = std::env::args().nth(2).map(|s| s.to_lowercase());
        let no_recurse = arg.as_deref() == Some("norec");
        walk_dir(path, no_recurse).expect("Failed to walk directory")
    } else {
        vec![PathBuf::from_str(&path).unwrap()]
    };

    //Process all target images
    for img in targets {
        processing::process(img);
    }
}

//Collect all images in the folder + subpaths
fn walk_dir<P: AsRef<Path>>(path: P, no_recurse: bool) -> Option<Vec<PathBuf>> {
    fn is_img<P: AsRef<Path>>(path: P) -> bool {
        let buf = PathBuf::from(path.as_ref());
        let ext = buf.extension().map(OsStr::to_string_lossy).unwrap().to_string();
        matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "bmp")
    }

    let mut targets = vec![];

    std::fs::read_dir(path).ok()?
        .filter_map(|entry| entry.ok())
        .for_each(|file| {
            let path = file.path();
            if !no_recurse && path.is_dir() {
                let subtargets = walk_dir(&path, true).unwrap_or(vec![]);
                targets.extend(subtargets.into_iter());
            } else if is_img(&path) {
                targets.push(path);
            }
        });

    Some(targets)
}