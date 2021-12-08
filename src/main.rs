mod depth_diffs;

fn main() {
    let depths = vec![10, 11, 9, 8, 23, 24, 24];
    println!("Changes: {:?}", depth_diffs::calculate_diffs(&depths));
    println!(
        "Number of increases: {}",
        depth_diffs::calculate_increase_count(&depths)
    );
}
