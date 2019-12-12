


fn meets_criteria(password: i32) -> bool {
    let string = password.to_string();
    let mut prev_digit = 0;
    let mut has_doubled_digit = false;
    let mut same_digit_run = 0;
    for (i, c) in string.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if i != 0 {
            if digit < prev_digit {
                // Decrease is not allowed
                return false;
            }
            if digit == prev_digit {
                same_digit_run += 1;
            } else {
                if same_digit_run == 1 {
                    has_doubled_digit = true;
                }
                same_digit_run = 0;
            }
        }
        prev_digit = digit;
    }
    if same_digit_run == 1 {
        has_doubled_digit = true;
    }
    return has_doubled_digit;
}


fn main() {
    let start = 165432;
//    let start = 111122;
    let end = 707912;

    let mut num_passwords = 0;
    for password in start .. end {
        if meets_criteria(password) {
            num_passwords += 1;
        }
    }
    println!("{:?}", num_passwords);
}
