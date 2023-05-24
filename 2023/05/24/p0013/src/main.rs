fn main() {
    println!("{:?}", roman_to_int("MCMXCIV".to_owned()));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut int_val = 0;
    let mut cur_idx = 0;
    while let Some(c) = s.chars().nth(cur_idx) {
        if c == 'I' {
            if let Some(next_c) = s.chars().nth(cur_idx + 1) {
                if next_c == 'V' {
                    int_val += 4;
                    cur_idx += 1;
                } else if next_c == 'X' {
                    int_val += 9;
                    cur_idx += 1;
                } else {
                    int_val += 1;
                }
            } else {
                int_val += 1;
            }
        } else if c == 'V' {
            int_val += 5;
        } else if c == 'X' {
            if let Some(next_c) = s.chars().nth(cur_idx + 1) {
                if next_c == 'L' {
                    int_val += 40;
                    cur_idx += 1;
                } else if next_c == 'C' {
                    int_val += 90;
                    cur_idx += 1;
                } else {
                    int_val += 10;
                }
            } else {
                int_val += 10;
            }
        } else if c == 'L' {
            int_val += 50;
        } else if c == 'C' {
            if let Some(next_c) = s.chars().nth(cur_idx + 1) {
                if next_c == 'D' {
                    int_val += 400;
                    cur_idx += 1;
                } else if next_c == 'M' {
                    int_val += 900;
                    cur_idx += 1;
                } else {
                    int_val += 100;
                }
            } else {
                int_val += 100;
            }
        } else if c == 'D' {
            int_val += 500;
        } else if c == 'M' {
            int_val += 1000;
        } else {
            panic!("out of boundary!");
        }
        cur_idx += 1;
    }
    int_val
}
