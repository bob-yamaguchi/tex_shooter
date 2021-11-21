use std::collections::HashMap;
use serde::{Deserialize, Serialize};
mod config;
use config::{ProjectSettings};
use wfd::{DialogParams, FOS_PICKFOLDERS};

#[derive(Serialize, Deserialize, Debug)]
struct RecieveInfo{
    id: i32,
    name: String,
    value: String,
    info: HashMap<String, String>
}

//static HTML_STR: &'static str = include_str!(concat!(env!("OUT_DIR"), "/main.html"));
static HTML_STR: &'static str = include_str!("../html/main.html");

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    let _ = std::env::set_current_dir(&path);
    path.push("main.html");
    let html = std::fs::read_to_string(&path).unwrap();

    let mut project_settings = ProjectSettings::load().unwrap();

    let w_view = web_view::builder()
        .title("Tex Shooter")
        .content(web_view::Content::Html(/*html*/HTML_STR))
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
                    path.push("rust_albedo.jpg");
                    let jpg = std::fs::read(&path).unwrap();
                    let _ = webview.eval(&format!("set_albedo(\"{}\")", base64::encode(&jpg)));
                    path.pop();
                    path.push("rust_normal.jpg");
                    let jpg = std::fs::read(&path).unwrap();
                    let _ = webview.eval(&format!("set_normal(\"{}\")", base64::encode(&jpg)));
                    path.pop();
                    path.push("rust_roughness.jpg");
                    let jpg = std::fs::read(&path).unwrap();
                    let _ = webview.eval(&format!("set_roughness(\"{}\")", base64::encode(&jpg)));
                }
                "request_root"=>{
                    let root_path = str::replace(project_settings.get_root_path(), "\\", "\\\\");
                    let _ = webview.eval(&format!("set_root(\"{}\")", root_path));
                }
                "change_root"=>{
                    let param = DialogParams{
                        options: FOS_PICKFOLDERS,
                        title: "select a root directory",
                        .. Default::default()
                    };
                    let result = wfd::open_dialog(param);
                    if result.is_ok(){
                        let path = result.unwrap().selected_file_path;
                        project_settings.set_root_path(path.to_str().unwrap());
                        project_settings.save();
                        let root_path = str::replace(project_settings.get_root_path(), "\\", "\\\\");
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

