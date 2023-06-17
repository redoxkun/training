fn high_and_low(numbers: &str) -> String {
    let (max, min) = numbers.split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::MIN, i32::MAX), |(max, min), x| (x.max(max), x.min(min)));
    format!("{} {}", max, min)
}

