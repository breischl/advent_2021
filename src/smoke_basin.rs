use crate::array_grid::ArrayGrid;
use itertools::Itertools;
use log::debug;

pub fn run(input: String) -> Result<String, String> {
    let width: i64 = input.lines().next().ok_or("No lines")?.len() as i64;
    let height: i64 = input.lines().count() as i64;
    let input: Vec<u8> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_string().parse::<u8>().unwrap()))
        .collect();
    let grid: ArrayGrid<u8> = ArrayGrid::create_from(width as usize, height as usize, input);

    let mut local_minima: Vec<u8> = vec![];

    for y in 0..height {
        for x in 0..width {
            let val = grid.get(x as usize, y as usize);
            let neighbors: Vec<&u8> = (vec![
                grid.get_checked(x - 1, y),
                grid.get_checked(x, y - 1),
                grid.get_checked(x + 1, y),
                grid.get_checked(x, y + 1),
            ] as Vec<Option<&u8>>)
                .into_iter()
                .filter_map(|x| x)
                .collect();
            let has_lower_neighbor = neighbors.iter().any(|&x| x <= val);

            debug!(
                "coords=({}, {}), val={}, neighbors={}, has_lower_neighbor={}",
                x,
                y,
                val,
                neighbors.iter().join(","),
                has_lower_neighbor
            );
            if !has_lower_neighbor {
                local_minima.push(*val);
            }
        }
    }

    debug!(
        "num_minima={}, points={}",
        local_minima.len(),
        local_minima.iter().join(",")
    );
    let risk: u32 = local_minima.iter().map(|&h| h as u32 + 1).sum();

    Ok(format!("risk={}", risk))
}
