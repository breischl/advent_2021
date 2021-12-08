mod depth_diffs;
use depth_diffs::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("inputs/day1.txt");
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
        Ok(_) => println!("Successfully read {} characters", s.len()),
    }

    let depths: Vec<u16> = s
        .lines()
        .map(|line| match line.parse() {
            Err(why) => panic!("Failed to parse {} to integer because {}", line, why),
            Ok(num) => num,
        })
        .collect();
    println!("Depths: {:?}", &depths);

    let depth_diffs = calculate_diffs(&depths);
    println!("Changes: {:?}", depth_diffs);
    println!(
        "Number of increases: {}",
        calculate_increase_count(&depth_diffs)
    );

    let windows = calculate_sliding_window_sums(&depths);
    let window_diffs = calculate_diffs(&windows);
    println!("Sliding windows: {:?}", windows);
    println!("Window changes: {:?}", window_diffs);
    println!(
        "Number of window increases: {}",
        calculate_increase_count(&window_diffs)
    );
}
