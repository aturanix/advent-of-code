use std::collections::{HashMap, HashSet};

fn parse_pair(input: &str) -> Option<(&str, &str)> {
    input.split_once('-')
}

fn is_ascii_uppercase(s: &str) -> bool {
    s.bytes().all(|x| x.is_ascii_uppercase())
}

fn parse_values(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut values: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut insert_pair = |from, to| {
        if let Some(x) = values.get_mut(from) {
            x.push(to);
        } else {
            values.insert(from, vec![to]);
        }
    };

    for (from, to) in input.split_terminator('\n').filter_map(parse_pair) {
        match (from, to) {
            ("start", _) | (_, "end") => insert_pair(from, to),
            (_, "start") | ("end", _) => insert_pair(to, from),
            (_, _) => {
                insert_pair(from, to);
                insert_pair(to, from);
            }
        }
    }
    values
}

fn count_end_path(
    mut visit_twice: bool,
    from: &str,
    values: &HashMap<&str, Vec<&str>>,
    visited: &HashSet<&str>,
) -> usize {
    visit_twice = visit_twice && !visited.contains(from);

    let visit_paths = |visited: &HashSet<&str>| {
        if let Some(to_values) = values.get(from) {
            to_values
                .iter()
                .filter(|x| visit_twice || !visited.contains(*x))
                .map(|x| count_end_path(visit_twice, x, values, visited))
                .sum()
        } else {
            0
        }
    };

    if from == "end" {
        1
    } else if is_ascii_uppercase(from) {
        visit_paths(visited)
    } else {
        let mut visited = visited.clone();
        visited.insert(from);
        visit_paths(&visited)
    }
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    let visited = HashSet::new();
    count_end_path(false, "start", &values, &visited)
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);
    let visited = HashSet::new();
    count_end_path(true, "start", &values, &visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve11() {
        let input = include_str!("../test1");
        assert_eq!(solve1(input), 10);
    }

    #[test]
    fn test_solve12() {
        let input = include_str!("../test2");
        assert_eq!(solve1(input), 19);
    }

    #[test]
    fn test_solve13() {
        let input = include_str!("../test3");
        assert_eq!(solve1(input), 226);
    }

    #[test]
    fn test_solve21() {
        let input = include_str!("../test1");
        assert_eq!(solve2(input), 36);
    }

    #[test]
    fn test_solve22() {
        let input = include_str!("../test2");
        assert_eq!(solve2(input), 103);
    }

    #[test]
    fn test_solve23() {
        let input = include_str!("../test3");
        assert_eq!(solve2(input), 3509);
    }
}
