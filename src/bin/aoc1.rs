fn read_input() -> impl Iterator<Item = i32> {
    let stdin = std::io::stdin();
    stdin.lines().filter_map(|l| {
        let l = l.ok()?;
        let sign = if l.starts_with("L") {
            -1
        } else if l.starts_with("R") {
            1
        } else {
            return None;
        };
        let value: i32 = l[1..].parse().unwrap();
        Some(value * sign)
    })
}

fn part1(moves: &[i32]) -> usize {
    let mut position = 50;
    let mut count = 0;
    for m in moves {
        position = (position + m).rem_euclid(100);
        if position == 0 {
            count += 1
        }
    }
    count
}

fn part2(moves: &[i32]) -> usize {
    let mut position = 50;
    let mut count = 0;
    for m in moves {
        if *m == 0 {
            continue;
        }
        let old_position = position;
        let unwrapped = position + m;
        position = (position + m).rem_euclid(100);
        tracing::debug!(
            "{} + {}{} -> {}",
            old_position,
            if *m < 0 { "L" } else { "R" },
            m.abs(),
            position
        );
        if unwrapped != position {
            // we crossed zero at least once; how many times was it, exactly?
            let turns = if old_position == 0 {
                m.abs() / 100
            } else if *m > 0 {
                (m - (100 - old_position)) / 100 + 1
            } else {
                (m.abs() - old_position) / 100 + 1
            };
            count += turns as usize;
            tracing::debug!(turns, count, "wrapped around");
        } else if position == 0 {
            count += 1;
            tracing::debug!(count, "landed at 0 naturally");
        }
    }
    count
}

fn main() {
    tracing_subscriber::fmt::init();
    let moves = read_input().collect::<Vec<_>>();
    println!("aoc1p1 = {:?}", part1(&moves));
    println!("aoc1p2 = {:?}", part2(&moves));
}

#[cfg(test)]
mod aoc1_tests {
    use super::part2;
    use test_log::test;

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(&[1000]), 10);
    }

    #[test]
    fn test_part2_rep() {
        assert_eq!(part2(&[1000, 1000, 50, 1000]), 31);
    }

    #[test]
    fn test_part2_lr() {
        assert_eq!(part2(&[-1000, 50]), 11);
    }

    #[test]
    fn test_part2_sweep() {
        for i in 150..250 {
            assert_eq!(part2(&[i]), 2);
        }
        for i in -249..=-150 {
            assert_eq!(part2(&[i]), 2);
        }
    }

    #[test]
    fn test_part2_reg() {
        assert_eq!(part2(&[-50, -562]), 6);
    }
}
