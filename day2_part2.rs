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
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            is_safe(&levels) || (0..levels.len()).any(|i| {
                let mut modified = levels.clone();
                modified.remove(i);
                is_safe(&modified)
            })
        })
        .count();

    println!("result: {}", safe_reports());
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diffs: Vec<i32> = levels
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    let increasing = diffs.iter().all(|&d| d > 0 && d <= 3);
    let decreasing = diffs.iter().all(|&d| (-3..0).contains(&d));

    increasing || decreasing
}
