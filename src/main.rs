mod array_grid;
mod binary;
mod dive;
mod hydrothermal;
mod lanternfish;
mod sonar;
mod squid;
mod whales;
mod seven_segment;

use clap::{AppSettings, ArgGroup, Parser};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args = Args::parse();

    let puzzle_info: PuzzleInfo;

    if let Some(day) = args.day {
        puzzle_info = get_puzzle_by_day_number(day);
    } else if let Some(p) = args.puzzle {
        puzzle_info = get_puzzle_info(&p);
    } else {
        puzzle_info = get_latest_puzzle();
    }
    let path = Path::new("inputs").join(puzzle_info.input);
    let path_display = path.display();
    //println!("Reading input from {}", path_display);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path_display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path_display, why),
        Ok(_) => (),
    }

    let result = (*puzzle_info.func)(s);
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}

fn get_puzzle_by_day_number(day: usize) -> PuzzleInfo {
    if let Some(p) = PUZZLES.get(day - 1) {
        get_puzzle_info(p)
    } else {
        panic!("Could not find puzzle for day {}", day);
    }
}

fn get_latest_puzzle() -> PuzzleInfo {
    let pz = PUZZLES;
    let last = pz
        .last()
        .expect("Puzzles list was unexpectedly empty, bad programmer!");
    get_puzzle_info(last)
}

fn get_puzzle_info(puzzle: &Puzzle) -> PuzzleInfo {
    match puzzle {
        Puzzle::Sonar => PuzzleInfo {
            input: String::from("sonar.txt"),
            func: &sonar::run,
        },
        Puzzle::Dive => PuzzleInfo {
            input: String::from("dive.txt"),
            func: &dive::run,
        },
        Puzzle::Binary => PuzzleInfo {
            input: String::from("binary.txt"),
            func: &binary::run,
        },
        Puzzle::Squid => PuzzleInfo {
            input: String::from("squid.txt"),
            func: &squid::run,
        },
        Puzzle::Hydrothermal => PuzzleInfo {
            input: String::from("hydrothermal.txt"),
            func: &hydrothermal::run,
        },
        Puzzle::Lanternfish => PuzzleInfo {
            input: String::from("lanternfish.txt"),
            func: &lanternfish::run,
        },
        Puzzle::Whales => PuzzleInfo {
            input: String::from("whales.txt"),
            func: &whales::run,
        },
        Puzzle::SevenSegment => PuzzleInfo {
            input: String::from("seven_segment.txt"),
            func: &seven_segment::run,
        },
        Puzzle::Latest => get_latest_puzzle(),
    }
}

struct PuzzleInfo {
    input: String,
    //Using `'static` lifetime since this is basically a commandline argument anyway
    func: &'static dyn Fn(String) -> Result<String, String>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(group(ArgGroup::new("puz").required(false).args(&["day", "puzzle"])))]
struct Args {
    #[clap(short, long, value_name = "DAY")]
    #[clap(parse(try_from_str))]
    //Run a puzzle by the day number of advent calendar (starting at 1)
    day: Option<usize>,

    #[clap(arg_enum)]
    #[clap(short, long)]
    puzzle: Option<Puzzle>,
}

#[derive(Copy, Clone, PartialEq, Eq, clap::ArgEnum, Debug)]
enum Puzzle {
    Latest,
    Sonar,
    Dive,
    Binary,
    Squid,
    Hydrothermal,
    Lanternfish,
    Whales,
    SevenSegment,
}

const PUZZLES: [Puzzle; 8] = [
    Puzzle::Sonar,
    Puzzle::Dive,
    Puzzle::Binary,
    Puzzle::Squid,
    Puzzle::Hydrothermal,
    Puzzle::Lanternfish,
    Puzzle::Whales,
    Puzzle::SevenSegment
];
