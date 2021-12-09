mod depth_diffs;
use depth_diffs::*;
use sliding_windows::Storage;
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
        Ok(_) => (),
    }

    let depths: Vec<u16> = s
        .lines()
        .map(|line| match line.parse() {
            Err(why) => panic!("Failed to parse {} to integer because {}", line, why),
            Ok(num) => num,
        })
        .collect();

    //For educational reasons I wrote this to avoid cloning or re-reading the entire list of depths, even though in this exact case it would not be problematic.
    //This led to the following somewhat-awkward cloning of iterators. Problem is that I want `calculate_direction()` to take either a plain iterator of depths,
    //or an iterator of the sliding windows produced from `calculate_sliding_window_sums()`. But the former would be iterating references, while the latter iterates owned values.
    //This is the best solution I could find that doesn't re-read the depths or re-allocate an equal (or almost-equal) amount of space.
    let mut depths_iter_1 = depths.into_iter();
    let mut depths_iter_2 = depths_iter_1.clone();

    let mut depth_diffs = calculate_direction(&mut depths_iter_1);
    let count = calculate_increase_count(&mut depth_diffs);
    println!("Number of increases: {}", count);

    let mut storage = Storage::new(3);
    let mut windows = calculate_sliding_window_sums(&mut depths_iter_2, &mut storage);
    let mut window_diffs = calculate_direction(&mut windows);
    let count = calculate_increase_count(&mut window_diffs);
    println!("Number of window increases: {}", count);
}
