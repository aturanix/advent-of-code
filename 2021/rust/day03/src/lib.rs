fn parse_values(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    let numbers_count = values.len();
    let number_length = values[0].len();

    let mut ones_counts = vec![0; number_length];
    for number in values.into_iter() {
        for (bit, ones) in number.bytes().zip(ones_counts.iter_mut()) {
            if bit == b'1' {
                *ones += 1;
            }
        }
    }

    let gamma = ones_counts
        .into_iter()
        .rev()
        .enumerate()
        .filter_map(|(pos, count)| {
            if ones_more_equal(numbers_count, count) {
                Some(pos)
            } else {
                None
            }
        })
        .fold(0, |acc, pos| acc + 2usize.pow(pos as u32));

    let epsilon =
        !gamma << (usize::BITS as usize - number_length) >> (usize::BITS as usize - number_length);
    gamma * epsilon
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);

    let o2 = sieve(values.clone(), ones_more_equal);
    let o2 = usize::from_str_radix(o2, 2).unwrap();

    let co2 = sieve(values, zeroes_more);
    let co2 = usize::from_str_radix(co2, 2).unwrap();

    o2 * co2
}

fn sieve(mut numbers: Vec<&str>, match_ones: impl Fn(usize, usize) -> bool) -> &str {
    let number_length = numbers[0].len();
    for i in 0..number_length {
        if numbers.len() == 1 {
            break;
        }

        let ones_count = numbers
            .iter()
            .filter(|x| *x.as_bytes().get(i).unwrap() == b'1')
            .count();

        let bit = if match_ones(numbers.len(), ones_count) {
            b'1'
        } else {
            b'0'
        };

        numbers = numbers
            .into_iter()
            .filter(|x| *x.as_bytes().get(i).unwrap() == bit)
            .collect();
    }
    numbers[0]
}

fn ones_more_equal(all: usize, ones: usize) -> bool {
    let zeroes = all - ones;
    ones >= zeroes
}

fn zeroes_more(all: usize, ones: usize) -> bool {
    !ones_more_equal(all, ones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 198);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 230);
    }
}
