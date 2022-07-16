enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_direction(s: &str) -> Option<Direction> {
    let tokens: Vec<_> = s.split_ascii_whitespace().collect();

    let value = tokens[1].parse().ok()?;

    match tokens[0] {
        "forward" => Some(Direction::Forward(value)),
        "down" => Some(Direction::Down(value)),
        "up" => Some(Direction::Up(value)),
        _ => None,
    }
}

fn parse_values(input: &str) -> Vec<Direction> {
    input
        .split_terminator('\n')
        .filter_map(parse_direction)
        .collect()
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);

    let (mut horizontal, mut depth) = (0, 0);
    for d in values.into_iter() {
        match d {
            Direction::Forward(x) => horizontal += x,
            Direction::Down(x) => depth += x,
            Direction::Up(x) => depth -= x,
        };
    }
    horizontal * depth
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);

    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for d in values.into_iter() {
        match d {
            Direction::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Direction::Down(x) => aim += x,
            Direction::Up(x) => aim -= x,
        };
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 150);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 900);
    }
}
