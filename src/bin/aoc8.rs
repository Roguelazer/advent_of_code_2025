use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct JunctionId(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct CircuitId(usize);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn distance(&self, other: &Point3) -> f64 {
        let xdist = (other.x as f64 - self.x as f64).powi(2);
        let ydist = (other.y as f64 - self.y as f64).powi(2);
        let zdist = (other.z as f64 - self.z as f64).powi(2);
        (xdist + ydist + zdist).sqrt()
    }
}

fn build_distances(
    by_id: &BTreeMap<JunctionId, Point3>,
) -> Vec<(JunctionId, JunctionId, OrderedFloat<f64>)> {
    let mut distances = by_id
        .keys()
        .cartesian_product(by_id.keys())
        .filter_map(|(lhs, rhs)| {
            if lhs >= rhs {
                None
            } else {
                let d = by_id[lhs].distance(&by_id[rhs]);
                Some((*lhs, *rhs, OrderedFloat(d)))
            }
        })
        .collect::<Vec<(JunctionId, JunctionId, OrderedFloat<f64>)>>();
    distances.sort_by_key(|(l, r, d)| (*d, *l, *r));
    distances
}

fn read_input() -> anyhow::Result<Vec<(JunctionId, Point3)>> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let s = std::io::read_to_string(stdin_lock)?;
    s.lines()
        .enumerate()
        .map(|(i, l)| {
            let mut iter = l.splitn(3, ',');
            let x = iter.next().unwrap().parse()?;
            let y = iter.next().unwrap().parse()?;
            let z = iter.next().unwrap().parse()?;
            Ok((JunctionId(i), Point3 { x, y, z }))
        })
        .collect()
}

fn part1(by_id: &BTreeMap<JunctionId, Point3>) -> usize {
    let distances = build_distances(by_id);
    let mut circuits: BTreeMap<JunctionId, CircuitId> = BTreeMap::new();
    let mut next_circuit_id = 0;
    for (l, r, _d) in distances.iter().take(1000) {
        let existing_left = circuits.get(l).copied();
        let existing_right = circuits.get(r).copied();
        match (existing_left, existing_right) {
            (None, None) => {
                let id = CircuitId(next_circuit_id);
                next_circuit_id += 1;
                tracing::debug!("creating new circuit {:?} with {:?} and {:?}", id, l, r);
                circuits.insert(*l, id);
                circuits.insert(*r, id);
            }
            (Some(e), None) => {
                tracing::debug!("adding {:?} to {:?}", r, e);
                circuits.insert(*r, e);
            }
            (None, Some(e)) => {
                tracing::debug!("adding {:?} to {:?}", l, e);
                circuits.insert(*l, e);
            }
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    tracing::debug!("merging {:?} and {:?}", lhs, rhs);
                    for item in circuits.values_mut() {
                        if *item == lhs {
                            *item = rhs;
                        }
                    }
                }
            }
        }
    }
    let counts = circuits.values().counts();
    counts.values().sorted().rev().take(3).product()
}

fn part2(by_id: &BTreeMap<JunctionId, Point3>) -> i64 {
    let distances = build_distances(by_id);
    let mut circuits: BTreeMap<JunctionId, CircuitId> = BTreeMap::new();
    let mut next_circuit_id = 0;
    for (iteration, (l, r, _d)) in distances.iter().enumerate() {
        let existing_left = circuits.get(l).copied();
        let existing_right = circuits.get(r).copied();
        match (existing_left, existing_right) {
            (None, None) => {
                let id = CircuitId(next_circuit_id);
                next_circuit_id += 1;
                tracing::debug!("creating new circuit {:?} with {:?} and {:?}", id, l, r);
                circuits.insert(*l, id);
                circuits.insert(*r, id);
            }
            (Some(e), None) => {
                tracing::debug!("adding {:?} to {:?}", r, e);
                circuits.insert(*r, e);
            }
            (None, Some(e)) => {
                tracing::debug!("adding {:?} to {:?}", l, e);
                circuits.insert(*l, e);
            }
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    tracing::debug!("merging {:?} and {:?}", lhs, rhs);
                    for item in circuits.values_mut() {
                        if *item == lhs {
                            *item = rhs;
                        }
                    }
                }
            }
        }
        let finished = circuits.values().all_equal() && circuits.len() == by_id.len();
        if finished {
            tracing::info!(iteration, "finished finally");
            let lp = &by_id[l];
            let rp = &by_id[r];
            return lp.x * rp.x;
        }
    }
    panic!("failed to find a solution");
}

fn main() {
    tracing_subscriber::fmt::init();
    let points = read_input().unwrap();
    let by_id = points.iter().cloned().collect::<BTreeMap<_, _>>();
    println!("part 1: {}", part1(&by_id));
    println!("part 2: {}", part2(&by_id));
}
