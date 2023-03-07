use crate::utils::read_line;


pub fn part1() {
    let lines = read_line(String::from("./src/inputs/day2.txt"));
    let mut total: u32 = 0;
    for line in lines {
        let l = line.unwrap();
        let mut g: Vec<u32> = l.split('x').map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
        g.sort();
        let h = g[0];
        let l = g[1];
        let w = g[2];
        let min_area = h * l;
        let total_area = 2 * h * l + 2 * l * w + 2 * h * w;
        total += min_area + total_area;
    }

    println!("{}", total);

}

pub fn part2() {
    let lines = read_line(String::from("./src/inputs/day2.txt"));
    let mut total: u32 = 0;
    for line in lines {
        let l = line.unwrap();
        let mut g: Vec<u32> = l.split('x').map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
        g.sort();
        let h = g[0];
        let l = g[1];
        let w = g[2];
        let ribbon = 2 * h + 2 * l;
        let present = h * l * w;
        total += ribbon + present;
    }

    println!("{}", total);
}

