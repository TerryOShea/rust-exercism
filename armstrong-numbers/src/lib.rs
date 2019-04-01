pub fn get_digits(mut num: u32) -> Vec<u32> {
    let mut digits = vec![];

    while num > 0 {
        let last_digit = num % 10;
        if last_digit > 0 {
            digits.push(last_digit);
        }
        num /= 10;
    }

    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let num_digits = digits.len() as u32;
    digits.into_iter().fold(0, |acc, d| acc + d.pow(num_digits)) == num
}
