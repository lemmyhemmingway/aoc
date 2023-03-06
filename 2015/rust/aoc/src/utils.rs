use std::fs;

pub fn read_file(file_path: String) -> Vec<char>{
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(value) => {
            return value.chars().collect();
        },
        Err(_) => panic!("File not found"),
    }

}
