use std::collections::HashSet;

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_point(s: &str) -> Option<(usize, usize)> {
    s.split_once(',')
        .and_then(|(l, r)| Some((l.parse().ok()?, r.parse().ok()?)))
}

fn parse_fold(s: &str) -> Option<Fold> {
    s.strip_prefix("fold along ")
        .and_then(|x| x.split_once('='))
        .and_then(|(axis, pos)| {
            let pos = pos.parse().ok()?;
            match axis {
                "x" => Some(Fold::X(pos)),
                "y" => Some(Fold::Y(pos)),
                _ => None,
            }
        })
}

fn parse_values(input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let mut iter = input.split_terminator('\n');
    let points = iter.by_ref().map_while(parse_point).collect();
    let folds = iter.filter_map(parse_fold).collect();
    (points, folds)
}

fn fold_points(points: &mut Vec<(usize, usize)>, fold: &Fold) {
    for (x, y) in points {
        match *fold {
            Fold::X(pos) if pos < *x => *x = pos - (*x - pos),
            Fold::Y(pos) if pos < *y => *y = pos - (*y - pos),
            _ => (),
        }
    }
}

pub fn solve1(input: &str) -> usize {
    let (mut points, folds) = parse_values(input);
    fold_points(&mut points, folds.first().unwrap());
    let unique_points: HashSet<_> = points.into_iter().collect();
    unique_points.len()
}

pub fn solve2(input: &str) -> String {
    let (mut points, folds) = parse_values(input);

    for fold in folds {
        fold_points(&mut points, &fold);
    }

    let unique_points: HashSet<_> = points.into_iter().collect();

    let y: usize = unique_points.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let x: usize = unique_points.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let paper_size = x * y;

    let mut result = vec![b'.'; paper_size + y];

    for i in (x..(paper_size + y)).step_by(x + 1) {
        result[i] = b'\n';
    }

    for (j, i) in unique_points.into_iter() {
        result[i * (x + 1) + j] = b'#';
    }

    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 17);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        let want = "#####
#...#
#...#
#...#
#####
";
        assert_eq!(solve2(input), want);
    }
}
