use std::ops::RangeInclusive;

fn sum_invalid_part1(range: RangeInclusive<i64>) -> i64 {
    range
        .filter(|n| {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let div = s.len() / 2;
                let first_half = &s[..div];
                let second_half = &s[div..];
                first_half == second_half
            } else {
                false
            }
        })
        .sum()
}

fn is_invalid<const N: usize>(s: &[u8]) -> bool {
    if s.len() <= N || s.len().is_multiple_of(N) {
        return false;
    }
    let mut i = s.chunks(N);
    let first = i.next().unwrap();
    i.all(|c| c == first)
}

fn sum_invalid_part2(range: RangeInclusive<i64>) -> i64 {
    range
        .filter(|n| {
            let ss = n.to_string();
            let s = ss.as_bytes();
            is_invalid::<5>(s)
                || is_invalid::<4>(s)
                || is_invalid::<3>(s)
                || is_invalid::<2>(s)
                || is_invalid::<1>(s)
        })
        .sum()
}

fn read_input() -> Vec<RangeInclusive<i64>> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let string = std::io::read_to_string(stdin_lock).unwrap();
    string
        .replace('\n', "")
        .split(",")
        .filter_map(|range| {
            if let Some((lower, upper)) = range.split_once('-') {
                let lower = lower.parse::<i64>().unwrap();
                let upper = upper.parse::<i64>().unwrap();
                Some(lower..=upper)
            } else {
                None
            }
        })
        .collect()
}

fn part1(ranges: &[RangeInclusive<i64>]) -> i64 {
    ranges.iter().cloned().map(sum_invalid_part1).sum()
}

fn part2(ranges: &[RangeInclusive<i64>]) -> i64 {
    ranges.iter().cloned().map(sum_invalid_part2).sum()
}

fn main() {
    tracing_subscriber::fmt::init();
    let ranges = read_input();
    println!("part 1: {}", part1(&ranges));
    println!("part 2: {}", part2(&ranges));
}
