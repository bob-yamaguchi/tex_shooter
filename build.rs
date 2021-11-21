use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&out_dir).join("../../..");
    let _ = fs::copy("./html/rust_albedo.jpg", root_dir.join("rust_albedo.jpg"));
    let _ = fs::copy("./html/rust_normal.jpg", root_dir.join("rust_normal.jpg"));
    let _ = fs::copy("./html/rust_roughness.jpg", root_dir.join("rust_roughness.jpg"));
}
