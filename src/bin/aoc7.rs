use aoclib::{DenseGrid, Point};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Start,
    #[default]
    Empty,
    Splitter,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '.' => Self::Empty,
            '^' => Self::Splitter,
            _ => panic!("What is {}", c),
        }
    }
}

fn read_input() -> anyhow::Result<DenseGrid<Cell>> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let s = std::io::read_to_string(stdin_lock)?;
    Ok(DenseGrid::from_input(&s, Cell::from_char))
}

fn part1(g: &DenseGrid<Cell>) -> usize {
    let mut beam_columns = BTreeSet::new();
    let mut splits = 0;
    for row in g.rows() {
        for (i, cell) in row.iter().enumerate() {
            if *cell == Cell::Start {
                beam_columns.insert(i);
            }
            if beam_columns.contains(&i) {
                if *cell == Cell::Splitter {
                    beam_columns.remove(&i);
                    splits += 1;
                    if i > 0 {
                        // create to the left
                        beam_columns.insert(i - 1);
                    }
                    if i < row.len() - 1 {
                        beam_columns.insert(i + 1);
                    }
                }
            }
        }
    }
    splits
}

fn part2(g: &DenseGrid<Cell>) -> usize {
    let start = g.find(&Cell::Start).unwrap();
    let mut cache = Arc::new(Mutex::new(BTreeMap::new()));
    cached_points_to_end_from(g, start + Point::new(0, 1), cache)
}

fn cached_points_to_end_from(
    g: &DenseGrid<Cell>,
    point: Point,
    cache: Arc<Mutex<BTreeMap<Point, usize>>>,
) -> usize {
    {
        let guard = cache.lock().unwrap();
        if let Some(value) = guard.get(&point) {
            return *value;
        }
    }
    let value = points_to_end_from(g, point, Arc::clone(&cache));
    cache.lock().unwrap().insert(point, value);
    value
}

fn points_to_end_from(
    g: &DenseGrid<Cell>,
    mut point: Point,
    cache: Arc<Mutex<BTreeMap<Point, usize>>>,
) -> usize {
    loop {
        match g.get(point) {
            Some(Cell::Empty) => point = point + Point::new(0, 1),
            Some(Cell::Splitter) => {
                let left = point + Point::new(-1, 0);
                let right = point + Point::new(1, 0);
                let mut sum = 0;
                if g.get(left) == Some(Cell::Empty) {
                    sum += cached_points_to_end_from(g, left, Arc::clone(&cache));
                }
                if g.get(right) == Some(Cell::Empty) {
                    sum += cached_points_to_end_from(g, right, Arc::clone(&cache));
                }
                return sum;
            }
            Some(Cell::Start) => return 0,
            None => return 1,
        }
    }
}

fn main() {
    let grid = read_input().unwrap();
    println!("part 1: {}", part1(&grid));
    println!("part 2: {}", part2(&grid));
}
