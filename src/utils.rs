use std;
use std::path::{Path, PathBuf};

const BASE_DIR_NAME: &'static str = "media";

pub fn is_dir(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(md) => md.is_dir(),
        Err(_) => false
    }
}

pub fn get_base_dir() -> Result<PathBuf, String> {
    let mut cur_dir = std::env::current_dir().unwrap();

    loop {
        cur_dir.push(BASE_DIR_NAME);
        if is_dir(&cur_dir.as_path()) {
            return Ok(cur_dir);
        }
        if !cur_dir.pop() || !cur_dir.pop() {
            break;
        }
    }
    Err(format!("not found base direcory: {}", BASE_DIR_NAME))
}
