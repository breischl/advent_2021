//A very bit-twiddly answer to Day 3
//It was fun mucking around with bits, but I suspect this is a wildly verbose and inefficient solution
//It probably would've been a whole lot easier to just work with the characters separately

const BIT_LENGTH: usize = 12; //all the inputs are actually 12 bits long
const NUM_MASK: u16 = 0x0FFF; //mask a u16 down to 12 bits

pub fn run(input: String) -> Result<String, String> {
    let binary_lines: Vec<u16> = input
        .lines()
        .map(parse_binary)
        .map(|r| r.expect("Failed to parse binary value"))
        .collect();

    let most_common_bits = find_most_common_bits(binary_lines.iter());
    println!("Most common bits: {:?}", most_common_bits);

    let gamma = bits_to_int(&most_common_bits);
    let epsilon = !gamma & NUM_MASK;
    let power = gamma as u64 * epsilon as u64;

    let oxy_power = find_lr_value(&binary_lines, true);
    let co2_rating = find_lr_value(&binary_lines, false);
    let lr_rating = oxy_power as u64 * co2_rating as u64;

    Ok(format!(
        "Gamma: {}, Epsilon: {}, Power: {}\nOxy: {}, CO2: {}, Life Support: {}",
        gamma, epsilon, power, oxy_power, co2_rating, lr_rating
    ))
}

fn find_lr_value(nums: &[u16], find_most_common: bool) -> u16 {
    let mut prefix: u16 = 0;
    let mut mask: u16 = 0;

    for bitidx in (0..=(BIT_LENGTH - 1)).rev() {
        let (zeroes, ones) = nums
            .iter()
            .filter(|n| is_prefix_match(&prefix, n, mask))
            .fold((0, 0), |(zero_count, one_count), val| {
                let bit = extract_bit(val, bitidx as u16);
                if bit == 1 {
                    ((zero_count), (one_count + 1))
                } else {
                    ((zero_count + 1), (one_count))
                }
            });

        if (zeroes + ones) == 1 {
            break;
        } else if zeroes + ones == 0 {
            //This is a big hack. If we got to zero matches, then unset the previous mask bit and we should be done.
            //Covers case where the last bit left us with a single 1 and a single 0
            mask &= !(1 << (bitidx - 1));
            break;
        } else if (find_most_common && ones >= zeroes)
            || (!find_most_common && ones < zeroes && ones > 0)
        {
            prefix |= 1 << bitidx;
        }
        mask |= 1 << bitidx;
    }

    let results: Vec<&u16> = nums
        .iter()
        .filter(|n| is_prefix_match(&prefix, n, mask))
        .collect();
    debug_assert_eq!(1, results.len());

    *results[0]
}

fn is_prefix_match(left: &u16, right: &u16, mask: u16) -> bool {
    (left & mask) == (right & mask)
}

fn bits_to_int(bits: &[u16; BIT_LENGTH]) -> u16 {
    let mut int = 0u16;
    for (i, bit) in bits.iter().enumerate() {
        let shifted = bit << (BIT_LENGTH - i - 1);
        int |= shifted;
    }
    int
}

#[allow(clippy::needless_range_loop)]
fn find_most_common_bits(ints: std::slice::Iter<u16>) -> [u16; BIT_LENGTH] {
    let mut counts: [u16; BIT_LENGTH] = [0; BIT_LENGTH];
    let mut num_items: u32 = 0;

    for int in ints {
        num_items += 1;
        for i in 0..counts.len() as u16 {
            counts[i as usize] += extract_bit(int, BIT_LENGTH as u16 - i - 1);
        }
    }

    let half = (num_items / 2) as u16;
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
    pub fn is_prefix_match_works() {
        assert_eq!(
            true,
            is_prefix_match(&0b1000000000000000, &0b1111000000000000, 0b1000000000000000)
        );

        assert_eq!(
            false,
            is_prefix_match(&0b1000000000000000, &0b1111000000000000, 0b1100000000000000)
        );

        assert_eq!(
            false,
            is_prefix_match(&0b1001000000000000, &0b1011000000000000, 0b1111000000000000)
        );
    }

    #[test]
    pub fn find_longest_matching_prefix_works() {
        let input: Vec<u16> = vec![
            "000000000100",
            "000000011110",
            "000000010110",
            "000000010111",
            "000000010101",
            "000000001111",
            "000000000111",
            "000000011100",
            "000000010000",
            "000000011001",
            "000000000010",
            "000000001010",
        ]
        .iter()
        .map(|s| parse_binary(s).unwrap())
        .collect();

        assert_eq!(23, find_lr_value(&input, true));
        assert_eq!(10, find_lr_value(&input, false));
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
        let input = parse_binary("10100011").unwrap();
        assert_eq!(1, extract_bit(&input, 0));
        assert_eq!(1, extract_bit(&input, 1));
        assert_eq!(0, extract_bit(&input, 2));
        assert_eq!(0, extract_bit(&input, 3));
        assert_eq!(0, extract_bit(&input, 4));
        assert_eq!(1, extract_bit(&input, 5));
        assert_eq!(0, extract_bit(&input, 6));
        assert_eq!(1, extract_bit(&input, 7));
    }

    #[test]
    pub fn extract_bit_works_30() {
        let input = 30;
        assert_eq!(0, extract_bit(&input, 0));
        assert_eq!(1, extract_bit(&input, 1));
        assert_eq!(1, extract_bit(&input, 2));
        assert_eq!(1, extract_bit(&input, 3));
        assert_eq!(1, extract_bit(&input, 4));
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
        assert_eq!(expected, find_most_common_bits(input.iter()));
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
