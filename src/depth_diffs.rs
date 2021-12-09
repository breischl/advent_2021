use sliding_windows::{IterExt, Storage};
use std::cmp::Ordering;

/// Calculate the direction between successive depth measurements in the given stream
/// TODO: This should probably return Option<Iterator> instead, to account for error cases (eg, 0 or 1 element in the iterator)
pub fn calculate_direction<'a>(
    depths: &'a mut dyn Iterator<Item = u16>,
) -> impl Iterator<Item = DepthDirection> + 'a {
    let mut prev: Option<u16> = None;

    let new_iter = depths
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
            return direction;
        })
        .flatten();

    new_iter
}

/// Calculate the number of times the given iterator contains DepthDirection::Up
pub fn calculate_increase_count(directions: &mut impl Iterator<Item = DepthDirection>) -> u16 {
    directions
        .filter(|c: &DepthDirection| *c == DepthDirection::Up)
        .count() as u16
}

/// Calculates the sum of the values in the stream on a sliding window.
/// The size of the sliding window is determined by the `Storage<>` passed in.
/// TODO: This should probably return `Option<Iterator<_>>` for error cases (eg, less than one window of data)
pub fn calculate_sliding_window_sums<'a>(
    depths: &'a mut impl Iterator<Item = u16>,
    storage: &'a mut Storage<u16>,
) -> impl Iterator<Item = u16> + 'a {
    depths
        .sliding_windows(storage)
        .map(|window| window.iter().map(|&x| x).sum())
}

#[derive(PartialEq, Debug)]
pub enum DepthDirection {
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
