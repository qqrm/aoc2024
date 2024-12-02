const INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

fn main() {
    let safe_reports = || INPUT
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() < 2 {
                return true;
            }

            let mut diffs = nums.windows(2).map(|w| w[1] - w[0]);

            let first_diff = diffs.next().unwrap();
            let direction = first_diff.signum();

            if first_diff == 0 || first_diff.abs() > 3 {
                return false;
            }

            for d in diffs {
                if d.signum() != direction || d == 0 || d.abs() > 3 {
                    return false;
                }
            }
            true
        })
        .count();

    println!("result: {}", safe_reports());
}
