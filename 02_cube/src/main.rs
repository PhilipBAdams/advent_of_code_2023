use std::cmp::{max, Ordering};
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use regex::{Regex, CaptureMatches};

#[derive(Eq, Debug)]
struct CubeSet
{
    red : u32,
    green: u32,
    blue: u32,
}

impl CubeSet
{
    fn TakeMax(&mut self, other : &Self)
    {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);
    }

    fn Power(&self) -> u32
    {
        return self.red*self.green*self.blue;
    }

}

#[derive(Debug)]
struct Game
{
    id : u32,
    maxSeen : CubeSet,
}

impl Ord for CubeSet
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if self.red > other.red { return Ordering::Greater; }
        if self.green > other.green { return Ordering::Greater; }
        if self.blue > other.blue { return Ordering::Greater; }

        if self.red < other.red { return Ordering::Less; }
        if self.green < other.green { return Ordering::Less; }
        if self.blue < other.blue { return Ordering::Less; }
        
        return Ordering::Equal
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CubeSet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_path = Path::new(file_path);
    println!("Input file: {}", file_path.display());

    let file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why.to_string()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let constraint = CubeSet { red: 12, green: 13, blue : 14 };
    let mut sum_valid_ids = 0;
    let mut sum_power = 0;

    for line in reader.lines()
    {
        let game = parse_game(line.expect("Unable to read line"));
        sum_power += game.maxSeen.Power();
        if game.maxSeen < constraint
        {
            sum_valid_ids += game.id;
        }
    }

    println!("Sum of valid IDs = {sum_valid_ids}");
    println!("Sum of powers = {sum_power}");

    //let test = String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    //println!("{:?}", parse_game(test));
}

fn parse_game(line : String) -> Game
{
    let game_id_regex = Regex::new("^Game ([0-9]+):").unwrap();
    let set_regex = Regex::new("(((?<red>[0-9]+) red)|((?<blue>[0-9]+) blue)|((?<green>[0-9]+) green))").unwrap();

    let mut out = Game { id : 0, maxSeen : CubeSet { red: 0, green: 0, blue: 0 } };

    out.id = game_id_regex.captures(&line).unwrap().get(1).unwrap().as_str().parse().expect("Not able to parse game ID");

    for cap_match in set_regex.captures_iter(&line)
    {
        let mut parsed_set = CubeSet { red: 0, green: 0, blue: 0 };

        //println!("{:?}", cap_match);
        parsed_set.red = match cap_match.name("red")
        {
            Some(s) => s.as_str().parse().expect("Unable to parse number of red"),
            None => 0, 
        };

        parsed_set.green = match cap_match.name("green")
        {
            Some(s) => s.as_str().parse().expect("Unable to parse number of green"),
            None => 0, 
        };

        parsed_set.blue = match cap_match.name("blue")
        {
            Some(s) => s.as_str().parse().expect("Unable to parse number of blue"),
            None => 0, 
        };

        out.maxSeen.TakeMax(&parsed_set);
    }

    return out;
}

