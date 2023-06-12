fn to_string(numbers: &[u8]) -> String {
    numbers.into_iter().fold(String::new(), |acc, x| {format!("{}{}", acc, x)})
}

fn create_phone_number(numbers: &[u8]) -> String {
    format!("({}) {}-{}", to_string(&numbers[..3]),
                          to_string(&numbers[3..6]),
                          to_string(&numbers[6..]))
}

