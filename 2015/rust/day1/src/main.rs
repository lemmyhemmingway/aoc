use std::fs;

fn read_file(file_path: String) -> Vec<char>{
    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(value) => {
            return value.chars().collect();
        },
        Err(_) => panic!("File not found"),
    }

}
fn part1() {
    let contents = read_file(String::from("./input.txt"));
    let mut up = 0;
    let mut down = 0;
    for content in contents {
        match content {
            '(' => up += 1,
            ')' => down += 1,
            _ => panic!("wtf")
        }
    }
    println!("{}", up - down);
}
fn part2() {
    let content = read_file(String::from("./input.txt"));
    let mut position = 1;
    let mut counter = 0;
    for item in content {
        counter += 1;
        if item == ')' {
            position -= 1;
        } else if item == '(' {
            position += 1;
        } 
        if position == 0 {
            println!("{}", counter);
            break
        }
    }
}
fn main() {
    part2();
    part1();
}
