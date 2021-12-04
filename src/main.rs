use std::collections::HashMap;
use serde::{Deserialize, Serialize};
mod config;
use config::{ProjectSettings};
use edsdk::wrap;

#[derive(Serialize, Deserialize, Debug)]
struct RecieveInfo{
    id: i32,
    name: String,
    value: String,
    info: HashMap<String, String>
}

struct Application{
    project: ProjectSettings,
    edsdk: wrap::Library,
    camera_device: Option<wrap::Camera>,
    camera_session: Option<wrap::Session>,
}

impl Application{
    pub fn new()->Self{
        let edsdk = wrap::Library::initialize();
        return Application{project: ProjectSettings::load().unwrap(), edsdk: edsdk.unwrap(), camera_device: None, camera_session: None};
    }
    // send project data to webview
    pub fn send_project_root<T>(&self, webview: &mut web_view::WebView<T>){
        let root_path = str::replace(self.project.get_root_path(), "\\", "\\\\");
        let _ = webview.eval(&format!("set_root(\"{}\")", root_path));
    }
    // send project data to webview
    pub fn send_project<T>(&self, webview: &mut web_view::WebView<T>){
        let json = serde_json::to_string(&self.project);
        let json_decoded = str::replace(json.unwrap().as_str(), "\\", "\\\\");
        let _ = webview.eval(&format!("set_project(\"{}\")", json_decoded));
    }
    // change projects root path
    pub fn change_project_root<T>(&mut self, webview: &mut web_view::WebView<T>){
        // TODO:have to use current root path
        let mut current_path = std::env::current_exe().unwrap();
        current_path.pop();
        let result = web_view::DialogBuilder::new(webview).choose_directory("select a project root directory", current_path);
        if result.is_ok(){
            let path = result.unwrap();
            if path.is_some(){
                let path = path.unwrap();
                self.project.set_root_path(path.to_str().unwrap());
                self.project.save();
                self.send_project_root(webview);
                //self.send_project(webview);
            }
        }
    }
    // send image
    pub fn send_image<T>(&self, webview: &mut web_view::WebView<T>, image_name: &str, func_name: &str)
    {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push(image_name);
        let jpg = std::fs::read(&path).unwrap();
        let _ = webview.eval(&format!("{}(\"{}\")", func_name, base64::encode(&jpg)));
    }
    pub fn connect_camera<T>(&mut self, webview: &mut web_view::WebView<T>){
        let mut devices = self.edsdk.get_device_list();
        if devices.len() > 0{
            self.camera_device = std::mem::replace(&mut devices[0], None);
            let _ = self.camera_device.as_ref().map(|dev|{
                let info = dev.get_device_info().unwrap();
                let _ = webview.eval(&format!("set_connection(\"{}\")", info.description));
                let fmt = format!("set_connection(\"{}\")", info.description);
                let _ = webview.eval(&fmt);
                //eprint!("{}", fmt)
            });
        }
        else{
            let _ = webview.eval(&format!("set_connection(\"disconnecting\")"));
        }
    }

    pub fn invoked<T>(&mut self, webview: &mut web_view::WebView<T>, arg: &str){
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
                self.send_image(webview, "rust_albedo.jpg", "set_albedo");
                self.send_image(webview, "rust_normal.jpg", "set_normal");
                self.send_image(webview, "rust_roughness.jpg", "set_roughness");
            }
            "request_root"=>{
                self.send_project_root(webview);
            }
            "request_connecting"=>{
                self.connect_camera(webview);
            }
            "change_root"=>{
                self.change_project_root(webview);
            }
            _=>{
                eprint!("unknown {:?}", arg)
            }
        }
    }
}



static HTML_STR: &'static str = include_str!("../html/main.html");

fn main() {
    let mut app = Application::new();
/*
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    let _ = std::env::set_current_dir(&path);
    path.push("main.html");
    let html = std::fs::read_to_string(&path).unwrap();
*/
    let w_view = web_view::builder()
        .title("Tex Shooter")
        .content(web_view::Content::Html(/*html*/HTML_STR))
        .size(1280, 720)
        .user_data(())
        .invoke_handler(|webview, arg| {
            app.invoked(webview, arg);
            Ok(())
        })
        .build()
        .unwrap();
    let _ = w_view.run();
}

