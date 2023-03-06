use crate::utils::read_file;

pub fn part1() {
    let contents = read_file(String::from("./src/inputs/input.txt"));
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
pub fn part2() {

    let content = read_file(String::from("./src/inputs/input.txt"));
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
