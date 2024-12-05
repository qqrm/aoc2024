const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn main() {
    let total = || -> i32 {
        let mut sum = 0;
        let chars: Vec<char> = INPUT.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len - 4 {
            if chars[i] == 'm' && chars[i + 1] == 'u' && chars[i + 2] == 'l' && chars[i + 3] == '('
            {
                let mut j = i + 4;
                let mut x = 0;
                let mut digits = 0;
                while j < len && chars[j].is_ascii_digit() && digits < 3 {
                    x = x * 10 + chars[j].to_digit(10).unwrap() as i32;
                    j += 1;
                    digits += 1;
                }

                if j < len && chars[j] == ',' {
                    j += 1;
                } else {
                    i += 1;
                    continue;
                }

                let mut y = 0;
                digits = 0;
                while j < len && chars[j].is_ascii_digit() && digits < 3 {
                    y = y * 10 + chars[j].to_digit(10).unwrap() as i32;
                    j += 1;
                    digits += 1;
                }

                if j < len && chars[j] == ')' {
                    sum += x * y;
                    i = j + 1;
                    continue;
                }
            }
            i += 1;
        }

        sum
    };

    println!("result: {}", total());
}
