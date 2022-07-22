use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Line(Point, Point);

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let y = self.0.y;
            self.iter_x().map(|x| Point { x, y }).collect()
        } else if self.is_vertical() {
            let x = self.0.x;
            self.iter_y().map(|y| Point { x, y }).collect()
        } else {
            self.iter_x()
                .zip(self.iter_y())
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }

    pub fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    pub fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    pub fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn iter_x(&self) -> Box<dyn Iterator<Item = i32>> {
        if self.0.x < self.1.x {
            Box::new(self.0.x..=self.1.x)
        } else {
            Box::new((self.1.x..=self.0.x).rev())
        }
    }

    fn iter_y(&self) -> Box<dyn Iterator<Item = i32>> {
        if self.0.y < self.1.y {
            Box::new(self.0.y..=self.1.y)
        } else {
            Box::new((self.1.y..=self.0.y).rev())
        }
    }
}

fn parse_point(s: &str) -> Option<Point> {
    let (x, y) = s.split_once(',')?;
    Some(Point {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
    })
}

fn parse_pair(s: &str) -> Option<Line> {
    let (p1, p2) = s.split_once("->")?;
    Some(Line(
        parse_point(p1.trim_end())?,
        parse_point(p2.trim_start())?,
    ))
}

fn parse_values(input: &str) -> Vec<Line> {
    input.lines().filter_map(parse_pair).collect()
}

fn overlap_count(values: Vec<Line>) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for p in values.iter().flat_map(Line::get_points) {
        if let Some(v) = map.get_mut(&p) {
            *v += 1;
        } else {
            map.insert(p, 1);
        }
    }
    map.into_values().filter(|x| *x > 1).count()
}

pub fn solve1(input: &str) -> usize {
    let values = parse_values(input);
    let values = values.into_iter().filter(|x| !x.is_diagonal()).collect();

    overlap_count(values)
}

pub fn solve2(input: &str) -> usize {
    let values = parse_values(input);

    overlap_count(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 5);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 12);
    }

    #[test]
    fn test_get_points() {
        let l = Line(Point { x: 1, y: 1 }, Point { x: 1, y: 3 });
        let got = l.get_points();
        assert_eq!(
            got,
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 }
            ]
        );

        let l = Line(Point { x: 9, y: 7 }, Point { x: 7, y: 7 });
        let got = l.get_points();
        assert_eq!(
            got,
            vec![
                Point { x: 9, y: 7 },
                Point { x: 8, y: 7 },
                Point { x: 7, y: 7 }
            ]
        );
    }
}
