fn main() {
    let input = include_str!("input.txt");
    let result: u32 = input.lines().map(|line| calibrate_line(line)).sum();
    println!("{result}")
}

fn calibrate_line(line: &str) -> u32 {
    let left = line.chars().find(|&x| x.is_digit(10)).unwrap();
    let right = line.chars().rfind(|&x| x.is_digit(10)).unwrap();

    let res: u32 = format!("{left}{right}").parse().unwrap();
    res
}