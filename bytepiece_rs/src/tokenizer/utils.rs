use std::fs::File;
use std::io::Read;



pub fn read_to_string(file_path: &str) -> String {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_path);
    let mut file = File::open(file_path.to_str().unwrap()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
