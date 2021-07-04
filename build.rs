use std::env;
use std::fs;
use std::path::Path;


fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let root_dir = Path::new(&out_dir).join("../../..");
    let _ = fs::copy("./html/main.html", root_dir.join("main.html"));
    let _ = fs::copy("./html/rust_icon.jpg", root_dir.join("rust_icon.jpg"));
}
