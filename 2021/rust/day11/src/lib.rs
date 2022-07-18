fn parse_values(input: &str) -> Vec<Vec<u8>> {
    input
        .split_terminator('\n')
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

fn flash(values: &mut [Vec<u8>], flashes: &mut [Vec<bool>], i: usize, j: usize) {
    if flashes[i][j] {
        return;
    }

    if values[i][j] != 9 {
        values[i][j] += 1;
        return;
    }

    values[i][j] = 0;
    flashes[i][j] = true;

    if let Some((i, j)) = up(i, j) {
        if let Some((i, j)) = left(i, j) {
            flash(values, flashes, i, j);
        }

        if let Some((i, j)) = right(values, i, j) {
            flash(values, flashes, i, j);
        }

        flash(values, flashes, i, j);
    }

    if let Some((i, j)) = down(values, i, j) {
        if let Some((i, j)) = left(i, j) {
            flash(values, flashes, i, j);
        }

        if let Some((i, j)) = right(values, i, j) {
            flash(values, flashes, i, j);
        }

        flash(values, flashes, i, j);
    }

    if let Some((i, j)) = left(i, j) {
        flash(values, flashes, i, j);
    }

    if let Some((i, j)) = right(values, i, j) {
        flash(values, flashes, i, j);
    }
}

fn step(values: &mut [Vec<u8>], flashes: &mut [Vec<bool>]) {
    for i in 0..values.len() {
        for j in 0..values[i].len() {
            flash(values, flashes, i, j);
        }
    }
}

pub fn solve1(input: &str) -> usize {
    let mut values = parse_values(input);
    let mut flashes = vec![vec![false; values[0].len()]; values.len()];
    let mut flash_count = 0;
    for _ in 0..100 {
        step(&mut values, &mut flashes);

        flash_count += flashes.iter().flatten().filter(|x| **x).count();

        for line in flashes.iter_mut() {
            line.fill(false);
        }
    }

    flash_count
}

pub fn solve2(input: &str) -> usize {
    let mut values = parse_values(input);
    let mut flashes = vec![vec![false; values[0].len()]; values.len()];

    let mut i = 1;
    loop {
        step(&mut values, &mut flashes);

        if flashes.iter().flatten().all(|x| *x) {
            return i;
        }

        for line in flashes.iter_mut() {
            line.fill(false);
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 1656);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 195);
    }
}
