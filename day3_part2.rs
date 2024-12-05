const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn main() {
    let calc = || -> i64 {
        let bytes: Vec<char> = INPUT.chars().collect();
        let mut total = 0;
        let mut mul_enabled = true;
        let mut i = 0;
        let len = bytes.len();

        while i < len {
            if bytes[i..].starts_with(&['m', 'u', 'l', '(']) {
                let mut j = i + 4;
                let mut x = String::new();
                while j < len && bytes[j].is_ascii_digit() && x.len() < 3 {
                    x.push(bytes[j]);
                    j += 1;
                }
                if j >= len || bytes[j] != ',' {
                    i += 1;
                    continue;
                }
                j += 1;
                let mut y = String::new();
                while j < len && bytes[j].is_ascii_digit() && y.len() < 3 {
                    y.push(bytes[j]);
                    j += 1;
                }
                if j >= len || bytes[j] != ')' {
                    i += 1;
                    continue;
                }
                if let (Ok(x_val), Ok(y_val)) = (x.parse::<i64>(), y.parse::<i64>()) {
                    if mul_enabled {
                        total += x_val * y_val;
                    }
                }
                i = j + 1;
                continue;
            }

            if bytes[i..].starts_with(&['d', 'o', '(', ')']) {
                mul_enabled = true;
                i += 4;
                continue;
            }

            if bytes[i..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')']) {
                mul_enabled = false;
                i += 7;
                continue;
            }

            i += 1;
        }

        total
    };
    
    println!("result: {}", calc());
}
