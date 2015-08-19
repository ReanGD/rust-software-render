use std::env;
use std::fs;

pub fn get_full_path(filename: &str) -> Result<String, String> {
    let mut cur_dir = env::current_dir().unwrap();
    
    loop {
        cur_dir.push("media");
        let is_dir = match fs::metadata(cur_dir.to_str().unwrap()) {
            Ok(md) => md.is_dir(),
            Err(_) => false
        };
        if is_dir {
            break;
        }
        if !cur_dir.pop() || !cur_dir.pop() {
            return Err(format!("not found \"media\" directory for filename {}", filename));
        }
    }
    cur_dir.push(filename);
    match cur_dir.as_path().to_str() {
        Some(path) => Ok(path.to_string()),
        None => Err(format!("error get full path for filename {}", filename))
    }
}
