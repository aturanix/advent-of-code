fn parse_values(input: &str) -> Vec<&str> {
    input.split_terminator('\n').collect()
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    let mut score = 0;
    let mut characters: Vec<u8> = Vec::new();
    for s in values.into_iter() {
        for c in s.bytes() {
            match c {
                b')' => {
                    if !matches!(characters.pop(), Some(b'(')) {
                        score += 3;
                        break;
                    }
                }
                b']' => {
                    if !matches!(characters.pop(), Some(b'[')) {
                        score += 57;
                        break;
                    }
                }
                b'}' => {
                    if !matches!(characters.pop(), Some(b'{')) {
                        score += 1197;
                        break;
                    }
                }
                b'>' => {
                    if !matches!(characters.pop(), Some(b'<')) {
                        score += 25137;
                        break;
                    }
                }
                v => characters.push(v),
            };
        }
        characters.clear();
    }
    score
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);
    let mut scores: Vec<usize> = Vec::new();
    let mut characters: Vec<u8> = Vec::new();
    'outer: for s in values.into_iter() {
        for c in s.bytes() {
            match c {
                b')' => {
                    if !matches!(characters.pop(), Some(b'(')) {
                        characters.clear();
                        continue 'outer;
                    }
                }
                b']' => {
                    if !matches!(characters.pop(), Some(b'[')) {
                        characters.clear();
                        continue 'outer;
                    }
                }
                b'}' => {
                    if !matches!(characters.pop(), Some(b'{')) {
                        characters.clear();
                        continue 'outer;
                    }
                }
                b'>' => {
                    if !matches!(characters.pop(), Some(b'<')) {
                        characters.clear();
                        continue 'outer;
                    }
                }
                v => characters.push(v),
            };
        }

        let mut score = 0;
        while let Some(v) = characters.pop() {
            score *= 5;
            score += match v {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => 0,
            }
        }
        scores.push(score);

        characters.clear();
    }

    let mid = scores.len() / 2;
    let (_, middle, _) = scores.select_nth_unstable(mid);
    *middle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 26397);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 288957);
    }
}
