use sliding_windows::{IterExt, Storage};
use std::cmp::Ordering;

pub fn run(input: String) -> Result<String, String> {
    let depths = input.lines().map(|line| match line.parse() {
        Err(why) => panic!("Failed to parse {} to integer because {}", line, why),
        Ok(num) => num,
    });

    //For educational reasons I wrote this to avoid cloning or re-reading the entire list of depths, even though in this exact case it would not be problematic.
    //This led to the following somewhat-awkward cloning of iterators. Problem is that I want `calculate_direction()` to take either a plain iterator of depths,
    //or an iterator of the sliding windows produced from `calculate_sliding_window_sums()`. But the former would be iterating references, while the latter iterates owned values.
    //This is the best solution I could find that doesn't re-read the depths or re-allocate an equal (or almost-equal) amount of space.
    let mut depths_iter_1 = depths;
    let mut depths_iter_2 = depths_iter_1.clone();

    let mut depth_diffs = calculate_direction(&mut depths_iter_1);
    let count = calculate_increase_count(&mut depth_diffs);
    println!("Number of increases: {}", count);

    let mut storage = Storage::new(3);
    let mut windows = calculate_sliding_window_sums(&mut depths_iter_2, &mut storage);
    let mut window_diffs = calculate_direction(&mut windows);
    let count = calculate_increase_count(&mut window_diffs);
    println!("Number of window increases: {}", count);

    Ok(String::from("Ran successfully"))
}

/// Calculate the direction between successive depth measurements in the given stream
/// TODO: This should probably return Option<Iterator> instead, to account for error cases (eg, 0 or 1 element in the iterator)
fn calculate_direction(
    depths: &mut dyn Iterator<Item = u16>,
) -> impl Iterator<Item = DepthDirection> + '_ {
    let mut prev: Option<u16> = None;

    depths
        .map(move |d: u16| {
            let direction = prev.map(|p| {
                let diff = d.partial_cmp(&p).expect("How is your depth NaN?");
                match diff {
                    Ordering::Greater => DepthDirection::Up,
                    Ordering::Equal => DepthDirection::NoChange,
                    Ordering::Less => DepthDirection::Down,
                }
            });
            prev = Some(d);
            direction
        })
        .flatten()
}

/// Calculate the number of times the given iterator contains DepthDirection::Up
fn calculate_increase_count(directions: &mut impl Iterator<Item = DepthDirection>) -> u16 {
    directions
        .filter(|c: &DepthDirection| *c == DepthDirection::Up)
        .count() as u16
}

/// Calculates the sum of the values in the stream on a sliding window.
/// The size of the sliding window is determined by the `Storage<>` passed in.
/// TODO: This should probably return `Option<Iterator<_>>` for error cases (eg, less than one window of data)
fn calculate_sliding_window_sums<'a>(
    depths: &'a mut impl Iterator<Item = u16>,
    storage: &'a mut Storage<u16>,
) -> impl Iterator<Item = u16> + 'a {
    depths
        .sliding_windows(storage)
        .map(|window| window.iter().copied().sum())
}

#[derive(PartialEq, Debug)]
enum DepthDirection {
    Up,
    Down,
    NoChange,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn empty() {
        let input = vec![];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    pub fn single() {
        let input = vec![1];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    pub fn down_1() {
        let input = vec![10, 9];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::Down, result[0]);
    }

    #[test]
    pub fn up_1() {
        let input = vec![50, 75];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::Up, result[0]);
    }

    #[test]
    pub fn no_change_1() {
        let input = vec![50, 50];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::NoChange, result[0]);
    }

    #[test]
    pub fn sequence_1() {
        let input = vec![50, 51, 52, 53, 49, 5, 4, 4, 4, 100];
        let result: Vec<_> = calculate_direction(&mut input.into_iter()).collect();
        let expectation = vec![
            DepthDirection::Up,
            DepthDirection::Up,
            DepthDirection::Up,
            DepthDirection::Down,
            DepthDirection::Down,
            DepthDirection::Down,
            DepthDirection::NoChange,
            DepthDirection::NoChange,
            DepthDirection::Up,
        ];
        assert_eq!(result, expectation);
    }

    #[test]
    pub fn count_sequence_1() {
        let input = vec![
            DepthDirection::Up,
            DepthDirection::Up,
            DepthDirection::Up,
            DepthDirection::Down,
            DepthDirection::Down,
            DepthDirection::Down,
            DepthDirection::NoChange,
            DepthDirection::NoChange,
            DepthDirection::Up,
        ];
        let result = calculate_increase_count(&mut input.into_iter());
        let expectation = 4;
        assert_eq!(result, expectation);
    }
}
