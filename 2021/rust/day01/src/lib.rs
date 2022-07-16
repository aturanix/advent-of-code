use std::str::FromStr;

fn parse_values<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve1(input: &str) -> usize {
    let values: Vec<u16> = parse_values(input);
    values.windows(2).filter(|x| x[1] > x[0]).count()
}

pub fn solve2(input: &str) -> usize {
    let values: Vec<u32> = parse_values(input);
    values
        .windows(3)
        .zip(values.windows(3).skip(1))
        .filter(|(first, second)| second.iter().sum::<u32>() > first.iter().sum())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 7);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 5);
    }
}
