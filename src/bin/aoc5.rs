use std::io::BufRead;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Db {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

impl Db {
    fn part1(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| self.fresh.iter().any(|r| r.contains(i)))
            .count()
    }

    fn part2(&self) -> u64 {
        let mut current: Option<RangeInclusive<u64>> = None;
        let mut ranges = vec![];
        for range in &self.fresh {
            if let Some(c) = current {
                assert!(range.start() >= c.start());
                if *range.start() > *c.end() {
                    ranges.push(c);
                    current = Some(range.clone());
                } else {
                    let new_start = std::cmp::min(*c.start(), *range.start());
                    let new_end = std::cmp::max(*c.end(), *range.end());
                    current = Some(new_start..=new_end);
                }
            } else {
                current = Some(range.clone());
            }
        }
        if let Some(current) = current.take() {
            ranges.push(current);
        }
        ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

fn read_input() -> anyhow::Result<Db> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut in_ingredients = false;
    let mut ingredients = vec![];
    let mut fresh = vec![];
    for line in stdin_lock.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            in_ingredients = true;
        } else if in_ingredients {
            ingredients.push(line.parse()?);
        } else {
            let Some((start, end)) = line.split_once('-') else {
                anyhow::bail!("invalid range {}", line);
            };
            let start = start.parse()?;
            let end = end.parse()?;
            fresh.push(start..=end);
        }
    }
    ingredients.sort();
    fresh.sort_unstable_by_key(|r| (*r.start(), *r.end()));
    Ok(Db { fresh, ingredients })
}

fn main() {
    tracing_subscriber::fmt::init();
    let input = read_input().unwrap();
    println!("part 1: {}", input.part1());
    println!("part 2: {}", input.part2());
}

#[cfg(test)]
mod aoc5_tests {}
