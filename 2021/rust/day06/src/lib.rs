const DAYS1: usize = 80;
const DAYS2: usize = 256;
const NUMBER_OF_TIMERS: usize = 9;
const NEW_FISH_TIMER: usize = 8;
const RESET_TIMER: usize = 6;

fn parse_values(input: &str) -> Vec<usize> {
    input
        .trim_end()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn calculate_fish(fish: &Vec<usize>, days: usize) -> usize {
    let mut fish_counts = vec![0; NUMBER_OF_TIMERS];
    for v in fish {
        fish_counts[*v] += 1;
    }

    for _ in 0..days {
        let new_count = fish_counts[0];
        fish_counts.copy_within(1.., 0);
        fish_counts[RESET_TIMER] += new_count;
        fish_counts[NEW_FISH_TIMER] = new_count;
    }
    fish_counts.into_iter().sum()
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    calculate_fish(&values, DAYS1)
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);
    calculate_fish(&values, DAYS2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 5934);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 26984457539);
    }
}
