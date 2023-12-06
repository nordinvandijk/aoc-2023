fn main() {
    let input = include_str!("input.txt");
    let schematic = Schematic::from_str(input);

    let mut result = 0;
    for (y, l) in schematic.data.iter().enumerate() {
        let mut number: String = String::new();
        let mut number_is_valid = false;
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() { 
                number.push(c.clone());
                if schematic.index_has_adjacent_symbol(x as isize, y as isize) { number_is_valid = true };
            }
            if !c.is_ascii_digit() || x == (schematic.width - 1) as usize { 
                if number_is_valid { result += number.parse::<i32>().unwrap(); }
                number.clear(); 
                number_is_valid = false 
            }
        }
    }
    
    println!("{result}");
}

#[derive(Debug)]
struct Schematic {
    width: u32,
    height: u32,
    data: Vec<Vec<char>>
}

impl Schematic {
    fn from_str(str: &str) -> Self {
        let height = str.lines().count() as u32;
        let width = str.lines().next().unwrap().chars().count() as u32;
        let data = str.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

        Schematic { 
            height, 
            width,
            data
        }
    }

    pub fn index(&self, x: isize, y: isize) -> char {
        if x < 0 || x > (self.width - 1) as isize || y < 0 || y > (self.height - 1) as isize {
            return '.';
        }
        self.data.get(y as usize).unwrap().get(x as usize).unwrap().clone()
    }

    pub fn index_has_adjacent_symbol(&self, x: isize, y: isize) -> bool {
        let adjacent_coords:[(isize, isize); 8] = [
            (x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y), (x+1, y),
            (x-1, y+1), (x, y+1), (x+1, y+1)
        ];
        let adjacent_chars = adjacent_coords.map(|(x, y)| self.index(x, y));

        adjacent_chars.iter().any(|x| x.is_ascii_punctuation() && x != &'.')
    }
}