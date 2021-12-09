mod dive;
mod sonar;
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
    println!("Reading input from {}", path_display);

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

    (*puzzle_info.func)(s);
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
        Puzzle::Latest => get_latest_puzzle(),
    }
}

struct PuzzleInfo {
    input: String,
    //Using `'static` lifetime since this is basically a commandline argument anyway
    func: &'static dyn Fn(String) -> (),
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
}

const PUZZLES: [Puzzle; 2] = [Puzzle::Sonar, Puzzle::Dive];
