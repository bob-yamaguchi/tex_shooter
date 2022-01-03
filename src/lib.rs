use std::collections::HashMap;
use std::path::{/*Path, */PathBuf};
use serde::{Deserialize, Serialize};
mod config;
use config::{ProjectSettings};
mod camera;
use edsdk::wrap;
//use edsdk::types;

#[derive(Serialize, Deserialize, Debug)]
struct RecieveInfo{
    id: i32,
    name: String,
    value: String,
    info: HashMap<String, String>
}

pub struct Application{
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
    // send error
    pub fn send_error<T>(&self, webview: &mut web_view::WebView<T>, title: &str, message: &str){
        let _ = webview.eval(&format!("error_msg(\"{}\", \"{}\")", title, message));
    }
    // send project data to webview
    pub fn send_project_root<T>(&self, webview: &mut web_view::WebView<T>){
        let root_path = str::replace(self.project.get_root_path(), "\\", "\\\\");
        let _ = webview.eval(&format!("set_root(\"{}\")", root_path));
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
    pub fn send_process_list<T>(&self, webview: &mut web_view::WebView<T>){
        let pathes = self.project.calc_process_list();
        let pathes: Vec<String> = pathes.iter().map(|p|format!("\"{}\"",p)).collect();
        let path_string = pathes.join(",");
        let _ = webview.eval(&format!("set_process_list(\"[{}]\")", path_string));
    }
    // receive iso changed
    pub fn receive_iso(&mut self, iso_speed: &str){
        self.project.set_iso(iso_speed);
        if self.camera_session.is_some(){
            let _ = self.camera_session.as_ref().map(|session|{
                session.set_iso_speed(camera::convert_iso(self.project.get_iso()))
            });
        }
    }
    // receive av changed
    pub fn receive_av(&mut self, aperture_value: &str){
        self.project.set_aperture_value(aperture_value);
        if self.camera_session.is_some(){
            let _ = self.camera_session.as_ref().map(|session|{
                session.set_av(camera::convert_av(self.project.get_aperture_value_as_str()))
            });
        }
    }
    // receive tv changed
    pub fn receive_tv(&mut self, time_value: &str){
        self.project.set_time_value(time_value);
        if self.camera_session.is_some(){
            let _ = self.camera_session.as_ref().map(|session|{
                session.set_tv(camera::convert_tv(self.project.get_time_value_as_str()))
            });
        }
    }
    // connect and open session
    pub fn connect_camera<T>(&mut self, webview: &mut web_view::WebView<T>){
        let mut devices = self.edsdk.get_device_list();
        if devices.len() > 0{
            self.camera_device = std::mem::replace(&mut devices[0], None);
            let session = self.camera_device.as_ref().map(|dev|{
                let info = dev.get_device_info().unwrap();
                let _ = webview.eval(&format!("set_connection(\"{}\")", info.description));
                let fmt = format!("set_connection(\"{}\")", info.description);
                let _ = webview.eval(&fmt);
                let session = dev.open_session();
                if session.is_ok(){
                    // TODO: set values from config
                    Some(session.unwrap())
                }
                else{
                    None
                }
            });
            self.camera_session = session.unwrap();
        }
        else{
            let _ = webview.eval(&format!("set_connection(\"disconnecting\")"));
        }
    }
    pub fn create_process<T>(&mut self, webview: &mut web_view::WebView<T>, process_name: &str){
        let result = self.project.create_process(process_name);
        if result.is_some(){
            self.select_process(webview, process_name);
        }
        else{
            self.send_error(webview, "failed to create process", &format!("couldn't make a dir {} or a setting file on that.", process_name));
        }
    }
    pub fn select_process<T>(&mut self, webview: &mut web_view::WebView<T>, process_name: &str){
        if self.project.set_last_processing(process_name){
            ;
        }
        else{
            self.send_error(webview, "failed to select process", &format!("process {} may not be valid.", process_name));
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
            "update_iso"=>{
                self.receive_iso(deserialized.value.as_str());
            }
            "update_av"=>{
                self.receive_av(deserialized.value.as_str());
            }
            "update_tv"=>{
                self.receive_tv(deserialized.value.as_str());
            }
            "create_process"=>{
                self.create_process(webview, deserialized.value.as_str());
            }
            "select_process"=>{
                self.select_process(webview, deserialized.value.as_str());
            }
            "request_processes"=>{
                self.send_process_list(webview);
            }
            "request_caribrations"=>{
                
            }
            _=>{
                eprint!("unknown {:?}", arg)
            }
        }
    }
}



