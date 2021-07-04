//#![windows_subsystem="windows"]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
mod config;
use config::{ProjectSettings};

#[derive(Serialize, Deserialize, Debug)]
struct RecieveInfo{
    id: i32,
    name: String,
    value: String,
    info: HashMap<String, String>
}

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    let _ = std::env::set_current_dir(&path);
    path.push("main.html");
    let html = std::fs::read_to_string(&path).unwrap();
    let w_view = web_view::builder()
        .title("Tex Shooter")
        .content(web_view::Content::Html(html))
        .size(1280, 720)
        .user_data(())
        .invoke_handler(|webview, arg| {
            let deserialized : RecieveInfo = serde_json::from_str(arg).unwrap();
            match deserialized.name.as_str(){
                "button"=>{
                    eprint!("pressed")
                }
                "menu"=>{
                    let checked = deserialized.info.get("checked");
                    if checked.is_some(){
                        match &*(checked.unwrap().as_str()){
                            "true"=>{
                                eprint!("menu {:?}", deserialized.value)
                            },
                            _=>{
                            }
                        }
                    }
                }
                "request_img"=>{
                    let mut path = std::env::current_exe().unwrap();
                    path.pop();
                    path.push("rust_icon.jpg");
                    let jpg = std::fs::read(&path).unwrap();
                    let _ = webview.eval(&format!("set_image(\"target\", \"{}\")", base64::encode(&jpg)));
                }
                "request_root"=>{
                    let settings = ProjectSettings::load();
                    if settings.is_ok(){
                        let root_path = str::replace(settings.unwrap().get_root_path(), "\\", "\\\\");
                        let _ = webview.eval(&format!("set_root(\"{}\")", root_path));
                    }
                }
                _=>{
                    eprint!("unknown {:?}", arg)
                }
            }
            Ok(())
        })
        .build()
        .unwrap();
    let _ = w_view.run();
}

