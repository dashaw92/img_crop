// #![windows_subsystem = "windows"]

use std::{ffi::OsStr, fs::metadata, path::{Path, PathBuf}, str::FromStr};

use algs::experimental::Experimental;

mod algs;
mod processing;

fn main() {
    //Load the target file/directory from the argument, or default to the CWD
    let path = std::env::args().nth(1).unwrap_or(".".to_owned());
    let meta = metadata(&path).expect("Couldn't get metadata for target.");

    //Program accepts a file path as argument 1:
    //A directory means the program should crop all images recursively (unless norec is passed)
    //A file will be processed on its own
    let targets = if meta.is_dir() {
        let arg = std::env::args().nth(2).map(|s| s.to_lowercase());
        //Don't recurse into subfolders at the path provided, only
        //crop images directly related to the path.
        let no_recurse = arg.as_deref() == Some("norec");

        //Scan the folder
        walk_dir(path, !no_recurse).expect("Failed to walk directory")
    } else {
        //Only this image will be cropped
        vec![PathBuf::from_str(&path).expect("Path is invalid (does the filesystem support this path???)")]
    };

    //Process all target images
    for img in targets {
        processing::process::<Experimental>(img);
    }
}

//Collect all images in the folder + subpaths
fn walk_dir<P: AsRef<Path>>(path: P, recurse: bool) -> Option<Vec<PathBuf>> {
    //A subpar determination of a file being an image or not.
    //My reason for creating this program was for a one-off
    //mass photograph-to-digital-scan-athon, so it suits
    //my needs. However, it is definitely possible to improve.
    //image supports many more formats than the ones listed,
    //and basing judgement solely off the extension is silly.
    fn is_img<P: AsRef<Path>>(path: P) -> bool {
        let buf = PathBuf::from(path.as_ref());
        let Some(ext) = buf.extension().map(OsStr::to_string_lossy).map(|ext| ext.to_string()) else {
            //Image has no extension. See above, this is a hacky determination.
            return false
        };

        matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "bmp")
    }

    //Recursively accumulate all images in the given path (or not, if recurse is false).
    let mut targets = vec![];

    std::fs::read_dir(path).ok()?
        .filter_map(|entry| entry.ok())
        .for_each(|file| {
            let path = file.path();

            if recurse && path.is_dir() {
                //If recurse and the path is a folder, recurse into the folder and append
                //the images found to the current image targets list.
                let subtargets = walk_dir(&path, true).unwrap_or(vec![]);
                targets.extend(subtargets.into_iter());
            } else if is_img(&path) {
                targets.push(path);
            }
        });

    Some(targets)
}