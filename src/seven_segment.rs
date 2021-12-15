use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/*
Part 2 is a little crazy. I think this is the basis of the decoder ring.env_logger
2 segments => #1
3 segments => #7
4 segments => #4
5 segments => #2 - shares 1 segment with #1, 2 segments with #4, 2 segments with #7
              #3 - shares 2 segments with #1, 3 segments with #4, 3 segments with #7
              #5 - shares 1 segment with #1, 3 segments with #4, 2 segments with #7
6 segments => #0 - shares 2 segments with #1, 3 segments with #4, 3 segments with #7
              #6 - shares 1 segment with #1, 3 segments with #4, 2 segments with #7
              #9 - shares 2 segments with #1, 4 segments with #4, 3 segments with #7
7 segments => #8

algorithm (on input side of line only)
1) sort words by length
2) Extract values for 2, 3, & 4 segments
3) For remaining segments, compute overlap with #1, #7 and #4 to determine values
*/

pub fn run(input: String) -> Result<String, String> {
    let io: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            log::debug!("parts={:?}", parts);
            let inputs: Vec<&str> = parts[0].split_whitespace().collect();
            let outputs: Vec<&str> = parts[1].split_whitespace().collect();
            (inputs, outputs)
        })
        .collect();

    let part1_sum = io
        .iter()
        .map(|(_inputs, outputs)| {
            outputs
                .iter()
                .filter(|&digit| match digit.len() {
                    2 | 3 | 4 | 7 => {
                        log::debug!("matching {}", digit);
                        true
                    }
                    _ => false,
                })
                .count()
        })
        .sum::<usize>();

    let sum: usize = io
        .iter()
        .map(|(inputs, outputs)| {
            let segment_map = infer_segment_map(inputs);
            let digit = outputs
                .iter()
                .map(|raw_out| {
                    let mut sorted_out: Vec<char> = raw_out.chars().collect();
                    sorted_out.sort_unstable_by(|a, b| a.cmp(b));
                    String::from_iter(sorted_out.iter())
                })
                .map(|out| {
                    if let Some(digit) = segment_map.get(&out) {
                        digit.to_string()
                    } else {
                        panic!("Unable to map segment {}", out);
                    }
                })
                .join("")
                .parse::<usize>()
                .unwrap();
            digit
        })
        .inspect(|num| {
            log::debug!("Out: {}", num);
        })
        .sum();

    Ok(format!("part1_out_digits={}, part2_sum={}", part1_sum, sum))
}

fn infer_segment_map(inputs: &Vec<&str>) -> HashMap<String, u8> {
    let mut inputs: Vec<String> = inputs
        .into_iter()
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            String::from_iter(chars.iter())
        })
        .collect();
    inputs.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    let mut results: Vec<SevenSegmentDigit> = Vec::with_capacity(10);

    debug_assert!(inputs[0].len() == 2);
    debug_assert!(inputs[1].len() == 3);
    debug_assert!(inputs[2].len() == 4);

    let num_1 = SevenSegmentDigit::new(inputs[0].clone(), 1);
    let num_7 = SevenSegmentDigit::new(inputs[1].clone(), 7);
    let num_4 = SevenSegmentDigit::new(inputs[2].clone(), 4);

    for segment_str in inputs.into_iter().skip(3) {
        let num_1_overlap = num_1.count_overlaps(&segment_str);
        let num_4_overlap = num_4.count_overlaps(&segment_str);
        let num_7_overlap = num_7.count_overlaps(&segment_str);
        let digit = match segment_str {
            num_8 if num_8.len() == 7 => SevenSegmentDigit::new(num_8, 8),
            num_2
                if num_2.len() == 5
                    && num_1_overlap == 1
                    && num_4_overlap == 2
                    && num_7_overlap == 2 =>
            {
                SevenSegmentDigit::new(num_2, 2)
            }
            num_3
                if num_3.len() == 5
                    && num_1_overlap == 2
                    && num_4_overlap == 3
                    && num_7_overlap == 3 =>
            {
                SevenSegmentDigit::new(num_3, 3)
            }
            num_5
                if num_5.len() == 5
                    && num_1_overlap == 1
                    && num_4_overlap == 3
                    && num_7_overlap == 2 =>
            {
                SevenSegmentDigit::new(num_5, 5)
            }
            num_0
                if num_0.len() == 6
                    && num_1_overlap == 2
                    && num_4_overlap == 3
                    && num_7_overlap == 3 =>
            {
                SevenSegmentDigit::new(num_0, 0)
            }
            num_6
                if num_6.len() == 6
                    && num_1_overlap == 1
                    && num_4_overlap == 3
                    && num_7_overlap == 2 =>
            {
                SevenSegmentDigit::new(num_6, 6)
            }
            num_9
                if num_9.len() == 6
                    && num_1_overlap == 2
                    && num_4_overlap == 4
                    && num_7_overlap == 3 =>
            {
                SevenSegmentDigit::new(num_9, 9)
            }
            _ => panic!("Unable to determine value for {}", segment_str),
        };

        results.push(digit);
    }

    results.push(num_1);
    results.push(num_7);
    results.push(num_4);

    let mut map: HashMap<String, u8> = HashMap::with_capacity(10);
    results.into_iter().for_each(|d7| {
        map.insert(d7.key, d7.digit);
    });
    map
}

struct SevenSegmentDigit {
    segments: HashSet<char>,
    key: String,
    digit: u8,
}

impl SevenSegmentDigit {
    fn new(key: String, digit: u8) -> SevenSegmentDigit {
        let segments: HashSet<char> = key.chars().collect();
        SevenSegmentDigit {
            segments,
            key,
            digit,
        }
    }

    fn count_overlaps(&self, other: &str) -> usize {
        other.chars().filter(|c| self.segments.contains(c)).count()
    }
}
