
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::DateTime;

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
    time_value: RationalValue,
}

impl Default for ExposureSettings {
    fn default() -> Self {
        ExposureSettings{iso: "100".to_string(), aperture_value: "4.0".to_string(), time_value: RationalValue::from_str("1/15")}
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
        let denominator = match self.time_value.denominator{
            15=>16,
            30=>32,
            60=>64,
            125=>128,
            250=>256,
            500=>512,
            1000=>1024,
            2000=>2048,
            4000=>4096,
            8000=>8192,
            16000=>16384,
            32000=>32768,
            64000=>65536,
            _=>self.time_value.denominator
        };
        let v = self.time_value.numarator as f32 / denominator as f32;
        return -v.log2();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingOptions{
    auto_trimming: bool,
    trim_point: [(f32, f32);4],
    tiling: bool,
    tiling_blend: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessingSettings{
    name: String,
    uuid: Uuid,
    time: String, // DateTime<chrono::Local>
    exposure: ExposureSettings,
    options: ProcessingOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSettings{
    root_path: String,
    last_exposure: ExposureSettings,
    last_processing: Uuid,
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
                let deserialized = serde_json::from_str(&*json);
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
                    root_path : path.as_path().display().to_string(),
                    last_exposure: ExposureSettings::new(),
                    last_processing : Uuid::nil()
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
        self.last_exposure.time_value
    }
    pub fn set_time_value(&mut self, tv: &RationalValue){
        self.last_exposure.time_value = *tv;
    }
}

