use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&out_dir).join("../../..");
    let _ = fs::copy("./html/rust_icon.jpg", root_dir.join("rust_icon.jpg"));
}
