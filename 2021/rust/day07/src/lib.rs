fn parse_values(input: &str) -> Vec<usize> {
    input
        .trim_end()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn solve1(input: &str) -> usize {
    let mut values = parse_values(input);

    let median = if values.len() % 2 == 0 {
        let high = values.len() / 2;
        let low = high - 1;

        let (_, &mut high_median, _) = values.select_nth_unstable(high);
        let (_, &mut low_median, _) = values.select_nth_unstable(low);
        (high_median + low_median) / 2
    } else {
        let mid = values.len() / 2;

        let (_, &mut median, _) = values.select_nth_unstable(mid);
        median
    };

    values
        .into_iter()
        .fold(0, |acc, x| acc + x.abs_diff(median))
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);
    let max = *values.iter().max().unwrap();
    let min = *values.iter().min().unwrap();
    (min..=max)
        .map(|x| {
            values.iter().fold(0, |acc, v| {
                let diff = v.abs_diff(x);
                acc + diff * (diff + 1) / 2
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 37);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 168);
    }
}
