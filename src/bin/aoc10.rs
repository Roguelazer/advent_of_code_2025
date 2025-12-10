use std::collections::{BTreeSet, VecDeque};

use good_lp::{Solution as _, SolverModel as _};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{one_of, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

#[derive(Debug, Clone)]
struct Machine {
    lights: Box<[bool]>,
    wirings: Vec<Vec<u8>>,
    joltages: Box<[u32]>,
}

impl Machine {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(Self::parse_lights, space1),
                terminated(Self::parse_wirings, space1),
                Self::parse_joltages,
            )),
            |(lights, wirings, joltages)| Self {
                lights: lights.into(),
                wirings,
                joltages: joltages.into(),
            },
        )(s)
    }

    fn parse_lights(s: &str) -> IResult<&str, Vec<bool>> {
        map(
            delimited(tag("["), many1(one_of(".#")), tag("]")),
            |lights| lights.into_iter().map(|l| l == '#').collect(),
        )(s)
    }

    fn parse_wirings(s: &str) -> IResult<&str, Vec<Vec<u8>>> {
        separated_list1(space1, Self::parse_wiring)(s)
    }

    fn parse_wiring(s: &str) -> IResult<&str, Vec<u8>> {
        delimited(
            tag("("),
            separated_list1(tag(","), nom::character::complete::u8),
            tag(")"),
        )(s)
    }

    fn parse_joltages(s: &str) -> IResult<&str, Vec<u32>> {
        delimited(
            tag("{"),
            separated_list1(tag(","), nom::character::complete::u32),
            tag("}"),
        )(s)
    }

    fn initial_lights(&self) -> Box<[bool]> {
        vec![false; self.lights.len()].into()
    }

    fn part1(&self) -> usize {
        let mut cache = BTreeSet::<Box<[bool]>>::new();

        let mut tasks = VecDeque::new();

        tasks.push_back((0, self.initial_lights()));

        while let Some((current, current_state)) = tasks.pop_front() {
            if current_state == self.lights {
                return current;
            } else {
                for w in &self.wirings {
                    let next = Self::apply_wiring_to_lights(&current_state, w);
                    if !cache.contains(&next) {
                        cache.insert(next.clone());
                        tasks.push_back((current + 1, next));
                    }
                }
            }
        }
        0
    }

    fn apply_wiring_to_lights(state: &[bool], wiring: &[u8]) -> Box<[bool]> {
        let mut state = state.to_owned();
        for wire in wiring {
            let index = (*wire) as usize;
            state[index] = !state[index];
        }
        state.into()
    }

    fn part2(&self) -> i32 {
        // construct a system of equations and solve it
        //
        // given a line like
        // (0,1) (1, 2) (1, 3) (0, 3) {3, 5, 6, 7}
        //
        // we have the following equations
        //
        // a + d = 3
        // a + b + c = 5
        // b = 6
        // c + d = 7
        //
        // and we want to minimize a + b + c + d
        //
        // each variable represents a button

        tracing::debug!(buttons=?self.wirings, joltages=?self.joltages, "solving a machine");

        let mut vars = good_lp::variables!();

        let mut button_variables = Vec::with_capacity(self.wirings.len());

        for button in self.wirings.iter() {
            let max_value = button
                .iter()
                .map(|b| self.joltages[(*b) as usize])
                .min()
                .unwrap();
            button_variables.push(vars.add(good_lp::variable().integer().min(0).max(max_value)));
        }

        let objective = button_variables
            .iter()
            .fold(good_lp::Expression::from_other_affine(0), |a, b| a + b);

        tracing::trace!(?objective, "minimizing objective");

        let mut model = vars.minimise(objective).using(good_lp::default_solver);

        for (i, j) in self.joltages.iter().enumerate() {
            tracing::trace!(joltage = j, index = i, "Evaluating joltage");
            let expr = self
                .wirings
                .iter()
                .enumerate()
                .filter_map(|(bi, wires)| {
                    if wires.iter().any(|bj| *bj as usize == i) {
                        Some(button_variables[bi].clone())
                    } else {
                        None
                    }
                })
                .fold(good_lp::Expression::from_other_affine(0), |a, b| a + b);
            let constraint = expr.eq(*j);

            tracing::trace!(?constraint, "Adding constraint");
            model.add_constraint(constraint);
        }

        let res = model.solve().unwrap();

        if !matches!(res.status(), good_lp::SolutionStatus::Optimal) {
            panic!("uh oh, got {:?}", res.status());
        }

        // check the solution
        #[cfg(debug_assertions)]
        {
            let mut applied = vec![0; self.joltages.len()];
            for (i, var) in button_variables.iter().enumerate() {
                tracing::trace!(i, ?var, val = res.value(*var), "backchecking");
                let count = res.value(*var).round() as u32;
                for wire in self.wirings[i].iter() {
                    applied[*wire as usize] += count;
                }
            }
            let applied: Box<[u32]> = applied.into();
            assert_eq!(applied, self.joltages);
        }

        let solution = button_variables
            .into_iter()
            .map(|var| res.value(var).round() as i32)
            .collect::<Vec<_>>();
        tracing::debug!(?solution, "got solution");
        solution.into_iter().sum()
    }
}

fn read_input() -> anyhow::Result<Vec<Machine>> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let s = std::io::read_to_string(stdin_lock)?;
    s.lines()
        .map(|line| {
            let (remainder, p) =
                Machine::parse(line).map_err(|e| anyhow::anyhow!("failed to parse: {:?}", e))?;
            if !remainder.is_empty() {
                anyhow::bail!("unhandled parse input: {}", remainder);
            }
            Ok(p)
        })
        .collect()
}

fn main() {
    tracing_subscriber::fmt::init();
    let input = read_input().unwrap();
    println!("part 1: {}", input.iter().map(|i| i.part1()).sum::<usize>());
    println!("part 2: {}", input.iter().map(|i| i.part2()).sum::<i32>());
}
