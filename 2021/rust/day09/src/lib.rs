use std::collections::{BinaryHeap, HashSet};

fn parse_values(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.bytes().map(|x| x - b'0').collect())
        .collect()
}

fn left(i: usize, j: usize) -> Option<(usize, usize)> {
    j.checked_sub(1).map(|j| (i, j))
}

fn right(values: &[Vec<u8>], i: usize, j: usize) -> Option<(usize, usize)> {
    j.checked_add(1)
        .filter(|j| *j < values[i].len())
        .map(|j| (i, j))
}

fn up(i: usize, j: usize) -> Option<(usize, usize)> {
    i.checked_sub(1).map(|i| (i, j))
}

fn down(values: &[Vec<u8>], i: usize, j: usize) -> Option<(usize, usize)> {
    i.checked_add(1)
        .filter(|i| *i < values.len())
        .map(|i| (i, j))
}

fn low_points(values: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    for i in 0..values.len() {
        for j in 0..values[i].len() {
            let point = values[i][j];
            if let Some((i, j)) = left(i, j) {
                if values[i][j] <= point {
                    continue;
                }
            }

            if let Some((i, j)) = right(values, i, j) {
                if values[i][j] <= point {
                    continue;
                }
            }

            if let Some((i, j)) = up(i, j) {
                if values[i][j] <= point {
                    continue;
                }
            }

            if let Some((i, j)) = down(values, i, j) {
                if values[i][j] <= point {
                    continue;
                }
            }

            vec.push((i, j));
        }
    }
    vec
}

fn basin_size(
    values: &Vec<Vec<u8>>,
    visited: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> usize {
    let point = values[i][j];
    if visited.contains(&(i, j)) || point == 9 {
        return 0;
    }
    visited.insert((i, j));

    let mut size = 1;
    if let Some((i, j)) = left(i, j) {
        if values[i][j] > point {
            size += basin_size(values, visited, i, j);
        }
    }

    if let Some((i, j)) = right(values, i, j) {
        if values[i][j] > point {
            size += basin_size(values, visited, i, j);
        }
    }

    if let Some((i, j)) = up(i, j) {
        if values[i][j] > point {
            size += basin_size(values, visited, i, j);
        }
    }

    if let Some((i, j)) = down(values, i, j) {
        if values[i][j] > point {
            size += basin_size(values, visited, i, j);
        }
    }
    size
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    low_points(&values)
        .into_iter()
        .map(|x| values[x.0][x.1] as usize + 1)
        .sum()
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basin_sizes: BinaryHeap<usize> = low_points(&values)
        .into_iter()
        .map(|(i, j)| basin_size(&values, &mut visited, i, j))
        .collect();
    let mut product = 1;
    for _ in 0..3 {
        product *= basin_sizes.pop().unwrap();
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 15);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 1134);
    }
}
