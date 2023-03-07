use std::{fs::{self, File}, io::{self, BufReader, BufRead}};

pub fn read_file(file_path: String) -> Vec<char>{
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(value) => {
            return value.chars().collect();
        },
        Err(_) => panic!("File not found"),
    }

}

pub fn read_line(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}
