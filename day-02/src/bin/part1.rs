use std::{collections::HashMap, u32};

fn main() {
    let bag = Bag::new(12, 13, 14);
    let input = include_str!("input.txt");
    
    let mut games: HashMap<u32, Vec<Round>> = HashMap::new();
    input.lines().for_each(|line| { 
            let (k, v) = line_to_game(line); 
            games.insert(k, v);
        }
    );

    let mut result: u32 = 0;
    games.iter().for_each(|game|  {
        let game_is_possible = game.1.iter().map(|x| bag.is_round_possible(x)).all(|x| x);
        if game_is_possible { result += game.0 }
        }
    );
   
    println!("{result}");
}

fn line_to_game(line: &str) -> (u32, Vec<Round>) {
    let mut game: Vec<Round> = vec![];

    let (id_as_str, rounds) = line.split_once(": ").unwrap();
    let id: u32 = id_as_str.replace("Game ", "").parse().unwrap();
    let rounds_as_str: Vec<&str> = rounds.split("; ").collect();
    
    for r in rounds_as_str {
        let mut round: Round = Round::default();

        let entries = r.split(", ").collect::<Vec<_>>();
        for e in entries {
            let (amount, color) = e.split_once(" ").unwrap();
            match color {
                "red" => round.red_cubes = amount.parse().unwrap(),
                "blue" => round.blue_cubes = amount.parse().unwrap(),
                "green" => round.green_cubes = amount.parse().unwrap(),
                _ => ()
            }
            
        }
        game.push(round);
    }

    (id, game)
}
struct Bag {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32
}

impl Bag {
    fn new(red: u32, green: u32, blue: u32 ) -> Self {
        Bag { red_cubes: red, green_cubes: green, blue_cubes: blue }
    }

    pub fn is_round_possible(&self, record: &Round) -> bool {
        self.blue_cubes >= record.blue_cubes && self.green_cubes >= record.green_cubes && self.red_cubes >= record.red_cubes
    }
}

#[derive(Debug, Default)]
struct Round {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32
}