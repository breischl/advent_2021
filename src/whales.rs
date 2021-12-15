pub fn run(input: String) -> Result<String, String> {
    let mut crabs: Vec<i32> = input
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    crabs.sort_unstable();

    let median: i32;
    if crabs.len() % 2 == 1 {
        median = crabs[crabs.len() / 2];
    } else {
        let a_idx = crabs.len() / 2;
        let b_idx = crabs.len() / 2 - 1;
        let a = crabs[a_idx];
        let b = crabs[b_idx];
        median = (a + b) / 2;
    }

    let median_fuel: i32 = crabs.iter().map(|c| (c - median).abs()).sum();

    let sum: i32 = crabs.iter().sum::<i32>();
    let raw_mean = sum as f64 / crabs.len() as f64;
    let mean_floor = raw_mean.floor() as i32;
    let mean_ceil = raw_mean.ceil() as i32;

    println!(
        "sum={}, raw_mean={}, mean_floor={}, mean_ceil={}",
        sum, raw_mean, mean_floor, mean_ceil
    );

    let fuel: (i32, i32) = crabs
        .iter()
        .map(|&start| {
            //triangle numbers: n*(n+1)/2
            let diff_floor = (start - mean_floor).abs() as f64;
            let diff_ceil = (start - mean_ceil).abs() as f64;
            let fuel_floor = (diff_floor * ((diff_floor + 1f64) / 2f64)) as i32;
            let fuel_ceil = (diff_ceil * ((diff_ceil + 1f64) / 2f64)) as i32;
            (fuel_floor, fuel_ceil)
        })
        .fold((0i32, 0i32), |(old_f, old_c), (new_f, new_c)| {
            (old_f + new_f, old_c + new_c)
        });
    let fuel_floor = fuel.0;
    let fuel_ceil = fuel.1;
    let min_fuel = fuel_floor.min(fuel_ceil);

    Ok(format!(
        "median: {}, median_fuel: {}, mean_floor: {}, mean_ceil: {}, fuel_floor: {}, fuel_ceil: {}, min_fuel: {}",
        median, median_fuel, mean_floor, mean_ceil, fuel_floor, fuel_ceil, min_fuel
    ))
}
