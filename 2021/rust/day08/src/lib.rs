use std::collections::HashMap;

fn parse_input_output(input: &str) -> Option<(&str, &str)> {
    input
        .split_once('|')
        .map(|(inp, outp)| (inp.trim_end(), outp.trim_start()))
}

fn parse_values(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|x| parse_input_output(x.trim_end()))
        .collect()
}

fn pattern_to_signal(pattern: &str) -> Option<u8> {
    let mut u8 = 0;
    for segment in pattern.bytes() {
        match segment {
            b'a' => u8 |= 1,
            b'b' => u8 |= 1 << 1,
            b'c' => u8 |= 1 << 2,
            b'd' => u8 |= 1 << 3,
            b'e' => u8 |= 1 << 4,
            b'f' => u8 |= 1 << 5,
            b'g' => u8 |= 1 << 6,
            _ => return None,
        }
    }
    Some(u8)
}

fn str_to_signals(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .flat_map(pattern_to_signal)
        .collect()
}

fn new_signal_to_number_converter(mut signals: Vec<u8>) -> HashMap<u8, u8> {
    const SEVEN_BITS: u8 = 0b111_1111;

    let mut converter = HashMap::new();
    debug_assert_eq!(signals.len(), 10);

    let pos_by_ones_count = |vec: &Vec<u8>, x| {
        vec.iter()
            .map(|c| c.count_ones())
            .position(|c| c == x)
            .unwrap()
    };

    let pos_by_and_match = |vec: &Vec<u8>, x: u8| vec.iter().position(|c| (c & x) == x).unwrap();

    let pos_by_match = |vec: &Vec<u8>, x: u8| vec.iter().position(|c| *c == x).unwrap();

    let one = signals.swap_remove(pos_by_ones_count(&signals, 2));
    converter.insert(one, 1);

    let seven = signals.swap_remove(pos_by_ones_count(&signals, 3));
    converter.insert(seven, 7);

    let four = signals.swap_remove(pos_by_ones_count(&signals, 4));
    converter.insert(four, 4);

    let eight = signals.swap_remove(pos_by_ones_count(&signals, 7));
    converter.insert(eight, 8);

    let nine = signals.swap_remove(pos_by_and_match(&signals, four | seven));
    converter.insert(nine, 9);

    let six = signals.swap_remove(pos_by_and_match(&signals, !one & SEVEN_BITS));
    converter.insert(six, 6);

    let five = signals.swap_remove(pos_by_match(&signals, six & nine));
    converter.insert(five, 5);

    let zero = signals.swap_remove(pos_by_and_match(&signals, seven | (!four & SEVEN_BITS)));
    converter.insert(zero, 0);

    let three = signals.swap_remove(pos_by_and_match(&signals, one | (!zero & SEVEN_BITS)));
    converter.insert(three, 3);

    let two = signals.pop().unwrap();
    converter.insert(two, 2);

    converter
}

pub fn solve1(input: &str) -> usize {
    parse_values(input)
        .into_iter()
        .map(|(_, x)| {
            x.split_ascii_whitespace()
                .filter(|v| matches!(v.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

pub fn solve2(input: &str) -> usize {
    parse_values(input)
        .into_iter()
        .map(|(inp, outp)| {
            let number_converter = new_signal_to_number_converter(str_to_signals(inp));
            str_to_signals(outp)
                .into_iter()
                .map(|x| *number_converter.get(&x).unwrap() as usize)
                .fold(0, |acc, x| (acc * 10) + x)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test");
        assert_eq!(solve1(input), 26);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test");
        assert_eq!(solve2(input), 61229);
    }
}
