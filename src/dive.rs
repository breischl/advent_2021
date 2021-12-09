pub fn run(input: String) -> Result<String, String> {
    let (pos, multiple) = run_internal(input)?;
    Ok(String::from(format!(
        "Range: {}, Depth: {}, multiple: {}",
        pos.range, pos.depth, multiple
    )))
}

fn run_internal(input: String) -> Result<(Position, u64), String> {
    let mut pos = Position { range: 0, depth: 0 };
    let commands_results = input.lines().map(parse_command);

    for result in commands_results {
        match result {
            Err(e) => return Err(e.to_string()),
            Ok(command) => match command {
                Command::Forward { distance: d } => pos.range += d,
                Command::Down { distance: d } => pos.depth += d,
                Command::Up { distance: d } => pos.depth -= d,
            },
        }
    }

    let multiple = pos.depth as u64 * pos.range as u64;
    Ok((pos, multiple))
}

fn parse_command(line: &str) -> Result<Command, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if let [direction, distance, ..] = parts.as_slice() {
        match distance.parse::<u16>() {
            Err(e) => Err(format!(
                "Unable to parse distance from \"{}\": {}",
                distance,
                e.to_string()
            )),
            Ok(distance) => match direction {
                direction if "forward".eq_ignore_ascii_case(direction) => {
                    Ok(Command::Forward { distance })
                }
                direction if "up".eq_ignore_ascii_case(direction) => Ok(Command::Up { distance }),
                direction if "down".eq_ignore_ascii_case(direction) => {
                    Ok(Command::Down { distance })
                }
                _ => Err(format!("Unable to understand direction \"{}\"", direction)),
            },
        }
    } else {
        Err(format!("Unable to parse command \"{}\"", line))
    }
}

struct Position {
    range: u16,
    depth: u16,
}

#[derive(PartialEq, Debug, Eq)]
enum Command {
    Forward { distance: u16 },
    Up { distance: u16 },
    Down { distance: u16 },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_command_ok() -> Result<(), String> {
        assert_eq!(parse_command("down 2")?, Command::Down { distance: 2 });
        assert_eq!(parse_command("up 77")?, Command::Up { distance: 77 });
        assert_eq!(
            parse_command("    forward    100   ")?,
            Command::Forward { distance: 100 }
        );

        Ok(())
    }

    #[test]
    pub fn parse_command_err() {
        parse_command("").expect_err("This is not OK!");
        parse_command("1 down").expect_err("This is not OK!");
        parse_command("up up and away").expect_err("This is not OK!");
        parse_command("up 99999999999").expect_err("This is not OK!");
    }

    #[test]
    pub fn run_works() {
        let input = "down 20\nforward 20\nup 10\n";
        let result = run_internal(String::from(input)).expect("Should not have failed");
        assert_eq!(20, result.0.range);
        assert_eq!(10, result.0.depth);
        assert_eq!(200, result.1);
    }
}
