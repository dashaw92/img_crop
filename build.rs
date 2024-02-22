#[cfg(windows)]
fn main() {
    use windres::Build;

    Build::new().compile("img_crop.rc").unwrap();
}

#[cfg(not(windows))]
fn main() {}