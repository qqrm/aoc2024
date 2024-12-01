const INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

fn main() {
    let similarity_score: i32 = {
        let (left_numbers, right_numbers): (Vec<i32>, Vec<i32>) = INPUT
            .trim()
            .lines()
            .filter_map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .filter_map(|n| n.parse::<i32>().ok());
                Some((nums.next()?, nums.next()?))
            })
            .unzip();

        let counts = right_numbers.iter().fold(
            std::collections::HashMap::new(),
            |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            },
        );

        left_numbers
            .iter()
            .map(|&num| num * counts.get(&num).unwrap_or(&0))
            .sum()
    };

    println!("result: {similarity_score}");
}
