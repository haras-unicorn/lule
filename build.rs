use fs_extra::dir::CopyOptions;
use std::{env, fs, path::Path};

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let root = Path::new(&cargo_manifest_dir);

    let src = root.join("assets");
    let dest = root.join("target").join(profile);

    fs::create_dir_all(dest.clone()).unwrap();
    fs_extra::dir::copy(
        src,
        dest,
        &CopyOptions {
            overwrite: true,
            ..Default::default()
        },
    )
    .unwrap();
}
