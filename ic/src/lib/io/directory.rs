use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;

pub struct Directory;

impl Directory {
    pub fn exists(path: &String) -> bool {
        Path::new(path).exists()
    }

    pub fn is_directory(path: &String) -> bool {
        Directory::exists(path) && Path::new(path).is_dir()
    }

    pub fn get_directory(directory: &String, deep: bool) -> HashMap<String, String> {
        let mut content: HashMap<String, String> = HashMap::new();
        let entries = fs::read_dir(directory).unwrap();
    
        for entry in entries {
            let path: PathBuf = entry.unwrap().path();
            let path_str = path.to_string_lossy().to_string();
            let file_name = path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
    
            if path.is_dir() && deep == true {
                content.extend(Directory::get_directory(&path_str, deep)); 
            } else {
                content.insert(file_name, path_str);
            }
        }
    
        return content;
    }
}
