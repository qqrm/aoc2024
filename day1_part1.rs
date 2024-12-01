const INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

fn main() {
    let (mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>) = INPUT
        .trim()
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let total_distance: i32 = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("result: {total_distance}");
}
