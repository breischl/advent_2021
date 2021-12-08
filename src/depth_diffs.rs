/// Take a vector DepthDirection and return the number of times the depth increases
pub fn calculate_increase_count(directions: &Vec<DepthDirection>) -> u16 {
    directions
        .iter()
        .filter(|c: &&DepthDirection| **c == DepthDirection::Up)
        .count() as u16
}

/// Take a vector of depths, and return a vector of changes in depth
/// The return vector will be shorter by one element, as it must be
/// If the input vector is less than 2 elements long, the result will contain the single element `DepthDirection::NoChange`
///
/// This isn't technically required by the Advent problem, but I wanted to play around with Rust `enum` a bit
pub fn calculate_diffs(depths: &Vec<u16>) -> Vec<DepthDirection> {
    if depths.len() < 2 {
        return vec![DepthDirection::NoChange];
    }

    let mut results: Vec<DepthDirection> = Vec::with_capacity(depths.len() - 1);
    for i in 1..depths.len() {
        let prev = depths[i - 1];
        let curr = depths[i];
        if curr > prev {
            results.push(DepthDirection::Up);
        } else if curr == prev {
            results.push(DepthDirection::NoChange);
        } else {
            results.push(DepthDirection::Down);
        }
    }
    results

    // This functional approach works, but the iterative version seems cleaner
    // let mut prev: Option<u16> = None;
    // depths
    //     .iter()
    //     .map(|d| {
    //         let direction = prev.map(|p| {
    //             if *d > p {
    //                 DepthDirection::Up
    //             } else if *d == p {
    //                 DepthDirection::NoChange
    //             } else {
    //                 DepthDirection::Down
    //             }
    //         });
    //         prev = Some(*d);
    //         return direction;
    //     })
    //     .filter(|opt| !opt.is_none())
    //     .map(Option::unwrap)
    //     .collect()
}

/// Calculates the sum of a 3-measurement sliding window, as described in Day 1, Problem 2
pub fn calculate_sliding_window_sums(depths: &Vec<u16>) -> Vec<u16> {
    if depths.len() < 3 {
        let result = match depths.len() {
            0 => vec![0],
            1 => vec![depths[0]],
            2 => vec![depths[0] + depths[1]],
            _ => unreachable!(),
        };
        return result;
    }

    let mut results: Vec<u16> = Vec::with_capacity(depths.len() - 2);
    for i in 2..depths.len() {
        results.push(depths[i] + depths[i - 1] + depths[i - 2]);
    }
    results
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
        let result = calculate_diffs(&vec![]);
        assert_eq!(result, vec![DepthDirection::NoChange]);
    }

    #[test]
    pub fn single() {
        let result = calculate_diffs(&vec![1]);
        assert_eq!(result, vec![DepthDirection::NoChange]);
    }

    #[test]
    pub fn down_1() {
        let result = calculate_diffs(&vec![10, 9]);
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::Down, result[0]);
    }

    #[test]
    pub fn up_1() {
        let result = calculate_diffs(&vec![50, 75]);
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::Up, result[0]);
    }

    #[test]
    pub fn no_change_1() {
        let result = calculate_diffs(&vec![50, 50]);
        assert_eq!(1, result.len());
        assert_eq!(DepthDirection::NoChange, result[0]);
    }

    #[test]
    pub fn sequence_1() {
        let result = calculate_diffs(&vec![50, 51, 52, 53, 49, 5, 4, 4, 4, 100]);
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
        let result = calculate_increase_count(&vec![50, 51, 52, 53, 49, 5, 4, 4, 4, 100]);
        let expectation = 4;
        assert_eq!(result, expectation);
    }
}
