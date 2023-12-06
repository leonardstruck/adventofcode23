fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();

    let sum = lines.iter().sum::<u32>();

    println!("Sum: {}", sum);
}

fn parse_line(line: &str) -> u32 {
    let digits = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap_or(first);

    // create new number from first and last digit
    first * 10 + last
}
