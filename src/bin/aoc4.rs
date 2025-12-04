use aoclib::{DenseGrid, HasEmpty};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Full,
}

impl HasEmpty for Cell {
    fn empty_value() -> Self {
        Self::Empty
    }
}

fn read_input() -> DenseGrid<Cell> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let input = std::io::read_to_string(stdin_lock).unwrap();
    DenseGrid::from_input(&input, |chr| match chr {
        '@' => Cell::Full,
        '.' => Cell::Empty,
        _ => panic!("what is {}", chr),
    })
}

fn compute_neighbors(grid: &DenseGrid<Cell>) -> DenseGrid<u8> {
    let mut adjacencies = DenseGrid::new_with_dimensions_from(grid, 0);
    for (coordinate, value) in grid.iter() {
        if value != Cell::Full {
            continue;
        }
        for neighbor in coordinate.all_neighbors_array() {
            if let Some(p) = adjacencies.get_mut(neighbor) {
                *p += 1;
            }
        }
    }
    adjacencies
}

fn part1(grid: &DenseGrid<Cell>) -> usize {
    let adjacencies = compute_neighbors(grid);
    grid.iter()
        .filter(|(coordinate, value)| {
            if *value == Cell::Full {
                adjacencies[*coordinate] < 4
            } else {
                false
            }
        })
        .count()
}

fn part2(mut grid: DenseGrid<Cell>) -> usize {
    let mut adjacencies = compute_neighbors(&grid);
    let mut removed = 0;
    loop {
        let mut removed_this_round = BTreeSet::new();
        for (coordinate, value) in grid.iter() {
            if value == Cell::Full {
                let count = adjacencies[coordinate];
                if count < 4 {
                    removed_this_round.insert(coordinate);
                }
            }
        }
        if removed_this_round.is_empty() {
            break;
        }
        tracing::debug!(
            removed_this_round = removed_this_round.len(),
            "removed some items",
        );
        removed += removed_this_round.len();
        for point in removed_this_round {
            for neighbor in point.all_neighbors_array() {
                if let Some(p) = adjacencies.get_mut(neighbor) {
                    *p -= 1
                }
            }
            grid[point] = Cell::Empty;
        }
    }
    removed
}

fn main() {
    tracing_subscriber::fmt::init();
    let grid = read_input();
    println!("part 1: {}", part1(&grid));
    println!("part 2: {}", part2(grid.clone()));
}

#[cfg(test)]
mod aoc4_tests {}
