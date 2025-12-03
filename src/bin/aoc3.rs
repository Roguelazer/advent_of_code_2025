use std::io::BufRead;

fn max_pos<T: Ord + Copy>(slice: &[T]) -> Option<(T, usize)> {
    let mut state = None;
    for (i, val) in slice.iter().enumerate() {
        if let Some((previous_max, _)) = state {
            if *val > previous_max {
                state = Some((*val, i))
            }
        } else {
            state = Some((*val, i))
        }
    }
    state
}

#[derive(Debug)]
struct Bank {
    num: usize,
    values: Vec<u64>,
}

impl Bank {
    fn parse(num: usize, bytes: &[u8]) -> Self {
        let inner = bytes
            .iter()
            .map(|item| {
                if *item > b'9' {
                    panic!("what is {}", item);
                } else {
                    (item - b'0') as u64
                }
            })
            .collect();
        Self { num, values: inner }
    }

    fn part1_score(&self) -> u64 {
        let (first, first_index) = max_pos(&self.values[..(self.values.len() - 1)]).unwrap();
        let (second, _) = max_pos(&self.values[(first_index + 1)..]).unwrap();
        let score = first * 10 + second;
        tracing::debug!(bank_num = self.num, score, "scored for part 1");
        score
    }

    fn part2_score(&self) -> u64 {
        let mut remainder = &self.values[..];
        let mut score = 0;
        for offset in (0..12).rev() {
            let end = remainder.len() - offset;
            let (first, first_index) = max_pos(&remainder[..end]).unwrap();
            remainder = &remainder[(first_index + 1)..];
            let scale = 10u64.pow(offset as u32);
            score += first * scale;
        }
        score
    }
}

fn read_input() -> Vec<Bank> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    stdin_lock
        .lines()
        .enumerate()
        .map(|(i, line)| Bank::parse(i, line.unwrap().as_bytes()))
        .collect()
}

fn part1(banks: &[Bank]) -> u64 {
    banks.iter().map(|b| b.part1_score()).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    banks.iter().map(|b| b.part2_score()).sum()
}

fn main() {
    tracing_subscriber::fmt::init();
    let banks = read_input();
    println!("part 1: {}", part1(&banks));
    println!("part 2: {}", part2(&banks));
}

#[cfg(test)]
mod aoc3_tests {}
