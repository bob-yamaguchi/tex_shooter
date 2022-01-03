use std::vec::Vec;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::{de, Deserialize, Serialize};
//use uuid::Uuid;
//use chrono::DateTime;
use super::camera;

pub fn save_to_json<T: Serialize>(instance: &T, file_name: &Path)->bool{
    let serialized = serde_json::to_string(instance);
    if serialized.is_ok(){
        let result = std::fs::write(file_name, &serialized.unwrap());
        return result.is_ok();
    }
    return false;
}

pub fn load_from_json<T: de::DeserializeOwned>(instance: &mut T, file_name: &Path)->bool{
    let file = File::open(file_name);
    if file.is_ok(){
        let reader = BufReader::new(file.unwrap());
        let deserialized = serde_json::from_reader(reader);
        if deserialized.is_ok(){
            *instance = deserialized.unwrap();
            return true;
        }
    }
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RationalValue{
    numarator : u32,
    denominator : u32,
}

impl ToString for RationalValue{
    fn to_string(&self)->String{
        if self.denominator == 1{
            self.numarator.to_string()
        }
        else{
            format!("{}/{}", self.numarator, self.denominator)
        }
    }
}

impl RationalValue{
    pub fn from_str(value : &str)->Self{
        let mut v = RationalValue{
            numarator:1,
            denominator:1,
        };
        v.set_from_str(value);
        return v;
    }
    pub fn set_from_str(&mut self, value : &str){
        if value.find("/").is_some(){
            let vec : Vec<&str> = value.split("/").collect();
            if vec.len() == 2{
                let v0 = vec[0].parse();
                let v1 = vec[1].parse();
                if v0.is_ok() && v1.is_ok(){
                    self.numarator = v0.unwrap();
                    self.denominator = v1.unwrap();
                }
            }
        }
        else{
            let v = value.parse();
            if v.is_ok(){
                self.numarator = v.unwrap();
                self.denominator = 1;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExposureSettings{
    iso: String,
    aperture_value: String,
    time_value: String,
}

impl Default for ExposureSettings {
    fn default()->Self {
        ExposureSettings{iso: "100".to_string(), aperture_value: "4.0".to_string(), time_value: "1/15".to_string()}
    }
}

impl ExposureSettings{
    pub fn new()->Self{
        Default::default()
    }
    pub fn calc_ev(&self)->f32{
        self.calc_iso() + self.calc_av() + self.calc_tv()
    }
    fn calc_iso(&self)->f32{
        let v : f32 = self.iso.parse().unwrap();
        return (v / 100.0).log2();
    }
    fn calc_av(&self)->f32{
        let v : f32 = self.aperture_value.parse().unwrap();
        return v.powi(2).log2();
    }
    fn calc_tv(&self)->f32{
        let tmp = RationalValue::from_str(&self.time_value);
        let denominator = camera::convert_tv_denominator(tmp.denominator);
        let v = tmp.numarator as f32 / denominator as f32;
        return -v.log2();
    }
}

const PROCESS_SETTING_FILE_NAME: &str = "process.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingOptions{
    auto_trimming: bool,
    trim_point: [(f32, f32);4],
    tiling: bool,
    tiling_blend: f32,
}
impl Default for ProcessingOptions {
    fn default()->Self {
        ProcessingOptions{
            auto_trimming: true,
            trim_point: [(0.0, 1.0), (0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            tiling: true,
            tiling_blend: 0.1
        }
    }
}
impl ProcessingOptions{
    pub fn new()->Self{
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LensCaribrationInfo{
    focal_length_wide: f32,
    focal_length_tale: f32,
    mat_wide: Vec<f32>,
    dist_wide: Vec<f32>,
    mat_tale: Vec<f32>,
    dist_tale: Vec<f32>,
}
impl Default for LensCaribrationInfo {
    fn default()->Self {
        LensCaribrationInfo{
            focal_length_wide: 0.0,
            focal_length_tale: 0.0,
            mat_wide: Vec::new(),
            dist_wide: Vec::new(),
            mat_tale: Vec::new(),
            dist_tale: Vec::new()
        }
    }
}
impl LensCaribrationInfo{
    pub fn new()->Self{
        Default::default()
    }
    pub fn save(&self, file_name: &Path)->bool{
        return save_to_json(self, file_name);
    }
    pub fn load(&mut self, file_name: &Path)->bool{
        return load_from_json(self, file_name);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LensInfo{
    name: String,       // lens name at the time of shooting
    focal_length: f32   // focal length at the time of shooting
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingState{
    lens: LensInfo,
    has_gray: bool,
    has_takes: [bool; 8]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingSettings{
    description: String,
    exposure: ExposureSettings,
    options: ProcessingOptions,
}
impl Default for ProcessingSettings {
    fn default()->Self {
        ProcessingSettings{
            description: "".to_string(),
            exposure: ExposureSettings::new(),
            options: ProcessingOptions::new()
        }
    }
}
impl ProcessingSettings{
    pub fn new()->Self{
        Default::default()
    }
    pub fn save(&self, file_name: &Path)->bool{
        return save_to_json(self, file_name);
    }
    pub fn load(&mut self, file_name: &Path)->bool{
        return load_from_json(self, file_name);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSettings{
    root_path: String,
    last_exposure: ExposureSettings,
    last_processing: String
}

impl ProjectSettings{
    fn get_json_path()->Result<std::path::PathBuf, String>{
        let path = dirs::home_dir();
        if path.is_some(){
            let mut path = path.unwrap();
            path.push("texshooter.json");
            return Ok(path);
        }
        Err(String::from("could not get a home directory."))
    }
    pub fn load()->Result<ProjectSettings, String>{
        // try to read a config
        let path = ProjectSettings::get_json_path();
        if path.is_ok(){
            let mut path = path.unwrap();
            let json = std::fs::read_to_string(&path);
            if json.is_ok(){
                let json = json.unwrap();
                let deserialized = serde_json::from_str(&json);
                if deserialized.is_ok(){
                    let settings : ProjectSettings = deserialized.unwrap();
                    return Ok(settings);
                }
                return Err(String::from("failed to deserialize settings"));
            }
            else{
                path.pop();
                path.push("texshooter");
                return Ok(ProjectSettings{
                    root_path: path.into_os_string().into_string().unwrap(),
                    //root_path: path.as_path().display().to_string(),
                    last_exposure: ExposureSettings::new(),
                    last_processing: "".to_string()
                });
            }
        }
        Err(path.unwrap_err())
    }
    pub fn save(&self){
        let serialized = serde_json::to_string(self);
        if serialized.is_ok(){
            let path = ProjectSettings::get_json_path();
            if path.is_ok(){
                let _ = std::fs::write(&path.unwrap(), &serialized.unwrap());
            }
        }
    }
    pub fn get_root_path(&self)->&str{
        self.root_path.as_str()
    }
    pub fn set_root_path(&mut self, path: &str){
        self.root_path = path.to_string();
    }
    pub fn get_iso(&self)->u32{
        self.last_exposure.iso.parse().unwrap()
    }
    pub fn get_iso_as_str(&self)->&str{
        &self.last_exposure.iso
    }
    pub fn set_iso(&mut self, iso: &str){
        self.last_exposure.iso = iso.to_string();
    }
    pub fn get_aperture_value(&self)->f32{
        self.last_exposure.aperture_value.parse().unwrap()
    }
    pub fn get_aperture_value_as_str(&self)->&str{
        &self.last_exposure.aperture_value
    }
    pub fn set_aperture_value(&mut self, av: &str){
        self.last_exposure.aperture_value = av.to_string();
    }
    pub fn get_time_value(&self)->RationalValue{
        RationalValue::from_str(&self.last_exposure.time_value)
    }
    pub fn get_time_value_as_str(&self)->&str{
        &self.last_exposure.time_value
    }
    pub fn set_time_value(&mut self, tv: &str){
        self.last_exposure.time_value = tv.to_string();
    }
    pub fn set_last_processing(&mut self, process_name: &str)->bool{
        if self.exist_process(process_name){
            self.last_processing = process_name.to_string();
            return true;
        }
        false
    }
    pub fn calc_process_list(&self)->Vec<String>{
        return Self::enum_subdir(&PathBuf::from(&self.root_path));
    }
    fn enum_subdir(dir_path: &Path)->Vec<String>{
        let mut process = Vec::new();
        let root_dir = dir_path.read_dir();
        if root_dir.is_ok(){
            let root_dir = root_dir.unwrap();
            for entry in root_dir{
                let path = entry.unwrap().path();
                if path.is_dir(){
                    let mut buf = path.to_path_buf();
                    buf.push(PROCESS_SETTING_FILE_NAME);
                    if buf.exists(){
                        process.push(path.into_os_string().into_string().unwrap());
                    }
                }
            }
        }
        process
    }
    pub fn create_process(&self, process_name: &str)->Option<ProcessingSettings>{
        let mut path = PathBuf::from(&self.root_path);
        path.push(process_name);
        if !path.exists() {
            let result = fs::create_dir(&path);
            if result.is_err(){
                return None;
            }
        }
        path.push(PROCESS_SETTING_FILE_NAME);
        let mut settings = ProcessingSettings::new();
        if !path.exists(){
            settings.save(&path);
            return Some(settings);
        }
        if settings.load(&path){
            return Some(settings);
        }
        None
    }
    fn exist_process(&self, process_name: &str)->bool{
        let mut path = PathBuf::from(&self.root_path);
        path.push(process_name);
        if !path.exists(){
            return false;
        }
        path.push(PROCESS_SETTING_FILE_NAME);
        return path.exists();
    }
}

