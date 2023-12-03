use std::env;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Captures;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_path = Path::new(file_path);
    println!("Input file: {}", file_path.display());

    let mut test = String::from("eightwo");
    test = eliminate_words(test);
    println!("{}", test);

    let file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why.to_string()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut sum : u32 = 0;

    for line in reader.lines()
    {
        sum += extract_calibration( line.expect("Failed to read line!"));
    }

    println!("Calibration sum: {}", sum);
}

fn extract_calibration(mut line : String) -> u32 
{
    line = eliminate_words(line);
    let left = get_digit(line.chars());
    let right = get_digit(line.chars().rev());
    return format!("{left}{right}").parse().expect("Failed at combining digits!");
}

fn get_digit(iter : impl Iterator<Item = char>) -> u32
{
    for c in iter
    {
        if c.is_digit(10)
        {
            return c.to_digit(10).expect("Failed convert to u32");
        }
    }
    panic!("Got to end of iterator without finding digit");
}

fn eliminate_words(mut line : String) -> String
{
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    while re.is_match(&line)
    {
        line = re.replace_all(&line, |caps: &Captures | 
            {
                format!("{}", convert_str_to_digit(&caps[1]))
            }).to_string();
    }
    return line;
}

fn convert_str_to_digit(word :&str) -> &str
{
    return match word
    {
        "zero" => "z0o",
        "one" => "o1e",
        "two" => "t2o",
        "three" => "t3e",
        "four" => "f4r",
        "five" => "f5e",
        "six" => "s6x",
        "seven" => "s7n",
        "eight" => "e8t",
        "nine" => "n9e",
        _ => panic!("Invalid input to convert_str_to_digit!"),
    }
}