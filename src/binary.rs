//A very bit-twiddly answer to Day 3
//It was fun mucking around with bits, but I suspect this is a wildly verbose and inefficient solution
const BIT_LENGTH: usize = 12; //all the inputs are actually 12 bits long
const NUM_MASK: u16 = 0x0FFF; //mask a u16 down to 12 bits

pub fn run(input: String) -> Result<String, String> {
    let binary_lines: Vec<u16> = input
        .lines()
        .map(parse_binary)
        .map(|r| r.expect("Failed to parse binary value"))
        .collect();

    let most_common_bits = find_most_common_bits(&binary_lines);

    println!("Most common bits: {:?}", most_common_bits);

    let gamma = bits_to_int(&most_common_bits);
    let epsilon = !gamma & NUM_MASK;
    let power = gamma as u64 * epsilon as u64;

    let oxy_power = find_longest_matching_prefix(&binary_lines, &gamma);
    let co2_rating = find_longest_matching_prefix(&binary_lines, &epsilon);
    let lr_rating = oxy_power as u64 * co2_rating as u64;

    Ok(String::from(format!(
        "Gamma: {}, Epsilon: {}, Power: {}\nOxy: {}, CO2: {}, Life Supprt: {}",
        gamma, epsilon, power, oxy_power, co2_rating, lr_rating
    )))
}

//Implements the Part 2 problem
//We are essentially trying to find the number from the input that has the longest matching prefix with gamma (to find oxy rating) or epsilon (to find CO2 rating)
//To do this we can walk through the list and check the matching prefix for each number
fn find_longest_matching_prefix(nums: &Vec<u16>, prefix: &u16) -> u16 {
    let mut best_match: u16 = nums[0]; //first item may not actually match, but given the input we can assume something else will later
    let mut best_match_length: u16 = count_prefix_match_bits(prefix, &nums[0]);

    for num in nums {
        let prefix_length = count_prefix_match_bits(prefix, num);
        if prefix_length > best_match_length {
            best_match = *num;
            best_match_length = prefix_length;
        }
    }

    return best_match;
}

fn count_prefix_match_bits(left: &u16, right: &u16) -> u16 {
    const BIT_LENGTH_16: u16 = BIT_LENGTH as u16;
    for i in 0..BIT_LENGTH_16 {
        let left_bit = extract_bit(left, BIT_LENGTH_16 - i - 1);
        let right_bit = extract_bit(right, BIT_LENGTH_16 - i - 1);
        if left_bit != right_bit {
            return i;
        }
    }
    return BIT_LENGTH_16;
}

fn bits_to_int(bits: &[u16; BIT_LENGTH]) -> u16 {
    let mut int = 0u16;
    for i in 0..BIT_LENGTH {
        let shifted = bits[i].clone() << (BIT_LENGTH - i - 1);
        int = int | shifted;
    }
    int
}

fn find_most_common_bits(ints: &Vec<u16>) -> [u16; BIT_LENGTH] {
    let mut counts: [u16; BIT_LENGTH] = [0; BIT_LENGTH];

    for int in ints {
        for i in 0..counts.len() as u16 {
            counts[i as usize] += extract_bit(int, BIT_LENGTH as u16 - i - 1);
        }
    }

    println!("Int counts: {:?}", counts);

    let half = (ints.len() / 2) as u16;
    for i in 0..counts.len() {
        if counts[i] >= half {
            counts[i] = 1;
        } else {
            counts[i] = 0;
        }
    }

    counts
}

fn extract_bit(num: &u16, pos: u16) -> u16 {
    (num >> pos) & 0x1
}

fn parse_binary(s: &str) -> Result<u16, String> {
    let b: u16 = u16::from_str_radix(s, 2).expect("Failed to parse string to u16 as binary");
    Ok(b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn find_longest_matching_prefix_works() {
        let input = vec![
            parse_binary("001000000001").unwrap(),
            parse_binary("101100000001").unwrap(),
            parse_binary("001010000000").unwrap(),
            parse_binary("101010000011").unwrap(),
        ];

        assert_eq!(
            input[3],
            find_longest_matching_prefix(&input, &parse_binary("101000000001").unwrap())
        );

        assert_eq!(
            input[1],
            find_longest_matching_prefix(&input, &parse_binary("101111111111").unwrap())
        );

        assert_eq!(
            input[2],
            find_longest_matching_prefix(&input, &parse_binary("001011100000").unwrap())
        );
    }

    #[test]
    pub fn count_prefix_match_bits_full_match() -> Result<(), String> {
        let left = parse_binary("010010001011")?;
        let right = parse_binary("010010001011")?;
        assert_eq!(12, count_prefix_match_bits(&left, &right));

        Ok(())
    }

    #[test]
    pub fn count_prefix_match_bits_no_match() -> Result<(), String> {
        let left = parse_binary("000000000000")?;
        let right = parse_binary("111111111111")?;
        assert_eq!(0, count_prefix_match_bits(&left, &right));

        Ok(())
    }

    #[test]
    pub fn count_prefix_match_bits_6() -> Result<(), String> {
        let left = parse_binary("101010000000")?;
        let right = parse_binary("101010111111")?;
        assert_eq!(6, count_prefix_match_bits(&left, &right));

        Ok(())
    }

    #[test]
    pub fn parse_binary_works() {
        assert_eq!(Ok(1u16), parse_binary("000000000001"));
        assert_eq!(Ok(2049u16), parse_binary("100000000001"));
        assert_eq!(Ok(128u16), parse_binary("000010000000"));
        assert_eq!(Ok(131u16), parse_binary("000010000011"));
    }

    #[test]
    pub fn extract_bit_works() {
        assert_eq!(1, extract_bit(&131, 0));
        assert_eq!(1, extract_bit(&131, 1));
        assert_eq!(0, extract_bit(&131, 2));
        assert_eq!(0, extract_bit(&131, 3));
        assert_eq!(0, extract_bit(&131, 4));
        assert_eq!(0, extract_bit(&131, 5));
        assert_eq!(0, extract_bit(&131, 6));
        assert_eq!(1, extract_bit(&131, 7));
    }

    #[test]
    pub fn find_mcb_works() {
        let input = vec![
            parse_binary("000000000001").unwrap(),
            parse_binary("100000000001").unwrap(),
            parse_binary("000010000000").unwrap(),
            parse_binary("000010000011").unwrap(),
        ];
        let expected: Vec<u16> = "000010000001"
            .chars()
            .map(|c| c.to_string().parse::<u16>().unwrap())
            .collect();
        assert_eq!(expected, find_most_common_bits(&input));
    }

    #[test]
    pub fn bits_to_int_works() {
        assert_eq!(3073, bits_to_int(&[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]));
        assert_eq!(0, bits_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(1, bits_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]));
        assert_eq!(9, bits_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]));
        assert_eq!(1033, bits_to_int(&[0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]));
    }
}
