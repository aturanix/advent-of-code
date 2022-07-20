use std::collections::HashMap;

struct Pairs<'a> {
    indices: HashMap<&'a str, usize>,
    two_pairs: Vec<(String, String)>,
    elements: Vec<u8>,
}

impl Pairs<'_> {
    fn new(pairs: Vec<&str>, elements: Vec<u8>) -> Pairs {
        let two_pairs: Vec<(String, String)> = pairs
            .iter()
            .zip(elements.iter())
            .map(|(&pair, &middle)| {
                let mut pair1: Vec<u8> = Vec::with_capacity(2);
                let mut pair2: Vec<u8> = Vec::with_capacity(2);

                pair1.push(pair.as_bytes()[0]);
                pair1.push(middle);

                pair2.push(middle);
                pair2.push(pair.as_bytes()[1]);

                let pair1 = unsafe { String::from_utf8_unchecked(pair1) };
                let pair2 = unsafe { String::from_utf8_unchecked(pair2) };

                (pair1, pair2)
            })
            .collect();

        let indices: HashMap<&str, usize> =
            pairs.into_iter().enumerate().map(|(x, y)| (y, x)).collect();

        Pairs {
            indices,
            two_pairs,
            elements,
        }
    }

    fn two_pairs(&self, pair: &str) -> (&str, &str) {
        let i = *self.indices.get(pair).unwrap();
        let (l, r) = &self.two_pairs[i];
        (l, r)
    }

    fn element(&self, pair: &str) -> u8 {
        let i = *self.indices.get(pair).unwrap();
        self.elements[i]
    }
}

fn parse_values(input: &str) -> (&str, HashMap<u8, usize>, HashMap<&str, usize>, Pairs) {
    let mut iter = input.split_terminator('\n');

    let template = iter.by_ref().next().unwrap();
    let (pairs, elements): (Vec<&str>, Vec<u8>) = iter
        .filter_map(|x| {
            x.split_once("->")
                .map(|(l, r)| (l.trim_end(), r.trim_start().as_bytes()[0]))
        })
        .unzip();

    let mut pairs_count: HashMap<&str, usize> = pairs.iter().map(|x| (*x, 0)).collect();
    for pair in template.as_bytes().windows(2) {
        let pair = unsafe { std::str::from_utf8_unchecked(pair) };
        if let Some(x) = pairs_count.get_mut(pair) {
            *x += 1;
        }
    }

    let mut elements_count: HashMap<u8, usize> = HashMap::new();
    for b in template.bytes() {
        if let Some(x) = elements_count.get_mut(&b) {
            *x += 1;
        } else {
            elements_count.insert(b, 1);
        }
    }

    (
        template,
        elements_count,
        pairs_count,
        Pairs::new(pairs, elements),
    )
}

fn insert(
    pairs_count: &mut HashMap<&str, usize>,
    elements_count: &mut HashMap<u8, usize>,
    pairs: &Pairs,
) {
    let current_pairs: Vec<&str> = pairs_count
        .iter()
        .filter(|(_, count)| **count > 0)
        .map(|(pair, _)| *pair)
        .collect();

    let mut to_increment: HashMap<&str, usize> = HashMap::new();
    let mut to_decrement: HashMap<&str, usize> = HashMap::new();

    for pair in current_pairs {
        let &pair_count = pairs_count.get(pair).unwrap();
        if let Some(x) = to_decrement.get_mut(pair) {
            *x += pair_count;
        } else {
            to_decrement.insert(pair, pair_count);
        }

        let element = pairs.element(pair);
        if let Some(x) = elements_count.get_mut(&element) {
            *x += pair_count;
        } else {
            elements_count.insert(element, pair_count);
        }

        let (left, right) = pairs.two_pairs(pair);

        if let Some(x) = to_increment.get_mut(left) {
            *x += pair_count;
        } else {
            to_increment.insert(left, pair_count);
        }

        if let Some(x) = to_increment.get_mut(right) {
            *x += pair_count;
        } else {
            to_increment.insert(right, pair_count);
        }
    }

    for (k, v) in to_increment.into_iter() {
        if let Some(x) = pairs_count.get_mut(k) {
            *x += v;
        }
    }

    for (k, v) in to_decrement.into_iter() {
        if let Some(x) = pairs_count.get_mut(k) {
            *x -= v;
        }
    }
}

pub fn solve1(input: &str) -> usize {
    let (_, mut elements_count, mut pairs_count, pairs) = parse_values(input);
    for _ in 0..10 {
        insert(&mut pairs_count, &mut elements_count, &pairs);
    }
    elements_count.values().max().unwrap() - elements_count.values().min().unwrap()
}

pub fn solve2(input: &str) -> usize {
    let (_, mut elements_count, mut pairs_count, pairs) = parse_values(input);
    for _ in 0..40 {
        insert(&mut pairs_count, &mut elements_count, &pairs);
    }
    elements_count.values().max().unwrap() - elements_count.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 1588);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 2188189693529);
    }
}
