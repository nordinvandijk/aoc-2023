fn main() {
    
    let input = include_str!("input.txt");
    let result: u32 = input.lines().map(|line| calibrate_line(line)).sum();
    println!("{result}");
}

fn calibrate_line(line: &str) -> u32 {
    let cleaned_line = line
    .replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e");
        
    let left = cleaned_line.chars().find(|&x| x.is_digit(10)).unwrap();
    let right = cleaned_line.chars().rfind(|&x| x.is_digit(10)).unwrap();

    let res: u32 = format!("{left}{right}").parse().unwrap();
    res
}
