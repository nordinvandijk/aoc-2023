use std::{collections::HashMap, u32};

fn main() {
    let input = include_str!("input.txt");
    
    let mut games: HashMap<u32, Vec<Round>> = HashMap::new();
    input.lines().for_each(|line| { 
            let (k, v) = line_to_game(line); 
            games.insert(k, v);
        }
    );

    let result: u32 = games.iter().map(|(_id, rounds)| {
        let mut highest_red = 0;
        let mut highest_blue = 0;
        let mut highest_green = 0;

        rounds.iter().for_each(|x| {
            if x.red_cubes > highest_red { highest_red = x.red_cubes }
            if x.blue_cubes > highest_blue { highest_blue = x.blue_cubes }
            if x.green_cubes > highest_green { highest_green = x.green_cubes }
        });

        highest_red * highest_blue * highest_green

    }).sum();
   
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

#[derive(Debug, Default)]
struct Round {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32
}