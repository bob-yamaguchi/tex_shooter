

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSettings{
    root_path : String,
}

impl ProjectSettings{
    pub fn load()->Result<ProjectSettings, String>{
        // try to read a config
        let path = dirs::home_dir();
        if path.is_some(){
            let mut path = path.unwrap();
            path.push("texshooter.json");
            let json = std::fs::read_to_string(&path);
            if json.is_ok(){
                let json = json.unwrap();
                let deserialized = serde_json::from_str(&*json);
                if deserialized.is_ok(){
                    let settings : ProjectSettings = deserialized.unwrap();
                    return Ok(settings);
                }
            }
            else{
                path.pop();
                path.push("texshooter");
                return Ok(ProjectSettings{
                    root_path : path.as_path().display().to_string(),
                });
            }
        }
        Err(String::from("could not get a home directory."))
    }
    pub fn get_root_path(&self)->&str{
        return self.root_path.as_str();
    }
}
