use image::EncodableLayout;
use std::env;
use std::fs;
use std::path::PathBuf;

macro_rules! build_img_asset {
    ($filepath:literal) => {
        let src_filepath = format!("assets/{}.png", $filepath);
        println!("cargo:rerun-if-changed={}", src_filepath);

        let dst_filepath = format!("assets/{}.bitmap", $filepath);

        let img = image::open(src_filepath).unwrap();
        let pixels = img.to_rgba8();

        fs::write(dst_filepath, pixels.as_bytes()).unwrap();
    };
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it.
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rerun-if-changed=build.rs");

    build_img_asset!("plus");
    build_img_asset!("biohazard");
}
