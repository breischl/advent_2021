pub fn run(input: String) -> Result<String, String> {
    let out_digits_count = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            //println!("parts={:?}", parts);
            let inputs: Vec<&str> = parts[0].split_whitespace().collect();
            let outputs: Vec<&str> = parts[1].split_whitespace().collect();
            (inputs, outputs)
        })
        .map(|(_inputs, outputs)| {
            outputs
                .iter()
                .filter(|&digit| match digit.len() {
                    2 | 3 | 4 | 7 => {
                        //println!("matching {}", digit);
                        true
                    }
                    _ => false,
                })
                .count()
        })
        .sum::<usize>();

    Ok(format!("out_digits={}", out_digits_count))
}
