use std::collections::HashMap;
use serde::{Deserialize, Serialize};
mod config;
use config::{ProjectSettings};
use edsdk::wrap;
use edsdk::types;

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

fn convert_iso(iso_str: u32)->types::ISOSpeed{
    match iso_str{
        50=>{types::ISOSpeed::ISO50}
        100=>{types::ISOSpeed::ISO100}
        200=>{types::ISOSpeed::ISO200}
        400=>{types::ISOSpeed::ISO400}
        800=>{types::ISOSpeed::ISO800}
        1600=>{types::ISOSpeed::ISO160}
        3200=>{types::ISOSpeed::ISO320}
        _=>{types::ISOSpeed::ISO100}
    }
}

fn convert_av(av_str: &str)->types::ApertureValue{
    match av_str{
        "1.0"=>{types::ApertureValue::Av1_0}
        "1.1"=>{types::ApertureValue::Av1_1}
        "1.2"=>{types::ApertureValue::Av1_2}
        "1.4"=>{types::ApertureValue::Av1_4}
        "1.6"=>{types::ApertureValue::Av1_6}
        "1.8"=>{types::ApertureValue::Av1_8}
        "2.0"=>{types::ApertureValue::Av2_0}
        "2.2"=>{types::ApertureValue::Av2_2}
        "2.5"=>{types::ApertureValue::Av2_5}
        "2.8"=>{types::ApertureValue::Av2_8}
        "3.2"=>{types::ApertureValue::Av3_2}
        "3.5"=>{types::ApertureValue::Av3_5}
        "4.0"=>{types::ApertureValue::Av4_0}
        "4.5"=>{types::ApertureValue::Av4_5}
        "5.0"=>{types::ApertureValue::Av5_0}
        "5.6"=>{types::ApertureValue::Av5_6}
        "6.3"=>{types::ApertureValue::Av6_3}
        "6.7"=>{types::ApertureValue::Av6_7}
        "7.1"=>{types::ApertureValue::Av7_1}
        "8.0"=>{types::ApertureValue::Av8_0}
        "9.0"=>{types::ApertureValue::Av9_0}
        "9.5"=>{types::ApertureValue::Av9_5}
        "10.0"=>{types::ApertureValue::Av10_0}
        "11.0"=>{types::ApertureValue::Av11_0}
        "13.0"=>{types::ApertureValue::Av13_0}
        "14.0"=>{types::ApertureValue::Av14_0}
        "16.0"=>{types::ApertureValue::Av16_0}
        "18.0"=>{types::ApertureValue::Av18_0}
        "19.0"=>{types::ApertureValue::Av19_0}
        "20.0"=>{types::ApertureValue::Av20_0}
        "22.0"=>{types::ApertureValue::Av22_0}
        "25.0"=>{types::ApertureValue::Av25_0}
        "27.0"=>{types::ApertureValue::Av27_0}
        "29.0"=>{types::ApertureValue::Av29_0}
        "32.0"=>{types::ApertureValue::Av32_0}
        _=>{types::ApertureValue::Av4_0}
    }
}

fn convert_tv(tv_str: &str)->types::ShutterSpeed{
    match tv_str{
        "3"=>{types::ShutterSpeed::Tv3}
        "2.5"=>{types::ShutterSpeed::Tv2_5}
        "2"=>{types::ShutterSpeed::Tv2}
        "1.6"=>{types::ShutterSpeed::Tv1_6}
        "1.5"=>{types::ShutterSpeed::Tv1_5}
        "1.3"=>{types::ShutterSpeed::Tv1_3}
        "1"=>{types::ShutterSpeed::Tv1}
        "0.8"=>{types::ShutterSpeed::Tv0_8}
        "0.7"=>{types::ShutterSpeed::Tv0_7}
        "0.6"=>{types::ShutterSpeed::Tv0_6}
        "0.5"=>{types::ShutterSpeed::Tv0_5}
        "0.4"=>{types::ShutterSpeed::Tv0_4}
        "0.3"=>{types::ShutterSpeed::Tv0_3}
        "1/4"=>{types::ShutterSpeed::Tv1_4th}
        "1/5"=>{types::ShutterSpeed::Tv1_5th}
        "1/6"=>{types::ShutterSpeed::Tv1_6th}
        "1/8"=>{types::ShutterSpeed::Tv1_8th}
        "1/10"=>{types::ShutterSpeed::Tv1_10th}
        "1/13"=>{types::ShutterSpeed::Tv1_13th}
        "1/15"=>{types::ShutterSpeed::Tv1_15th}
        "1/20"=>{types::ShutterSpeed::Tv1_20th}
        "1/25"=>{types::ShutterSpeed::Tv1_25th}
        "1/30"=>{types::ShutterSpeed::Tv1_30th}
        "1/40"=>{types::ShutterSpeed::Tv1_40th}
        "1/45"=>{types::ShutterSpeed::Tv1_45th}
        "1/50"=>{types::ShutterSpeed::Tv1_50th}
        "1/60"=>{types::ShutterSpeed::Tv1_60th}
        "1/80"=>{types::ShutterSpeed::Tv1_80th}
        "1/90"=>{types::ShutterSpeed::Tv1_90th}
        "1/100"=>{types::ShutterSpeed::Tv1_100th}
        "1/125"=>{types::ShutterSpeed::Tv1_125th}
        "1/160"=>{types::ShutterSpeed::Tv1_160th}
        "1/180"=>{types::ShutterSpeed::Tv1_180th}
        "1/200"=>{types::ShutterSpeed::Tv1_200th}
        "1/250"=>{types::ShutterSpeed::Tv1_250th}
        _=>{types::ShutterSpeed::Tv1_15th}
    }
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
    // receive iso
    pub fn receive_iso(&mut self, iso_speed: &str){
        self.project.set_iso(iso_speed);
        if self.camera_session.is_some(){
            let _ = self.camera_session.as_ref().map(|session|{
                session.set_iso_speed(convert_iso(self.project.get_iso()))
            });
        }
    }
    // receive av
    pub fn receive_av(&mut self, aperture_value: &str){
        self.project.set_aperture_value(aperture_value);
        if self.camera_session.is_some(){
            let _ = self.camera_session.as_ref().map(|session|{
                session.set_av(convert_av(self.project.get_aperture_value_as_str()))
            });
        }
    }
    //
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

