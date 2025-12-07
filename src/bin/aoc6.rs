use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "*" => Some(Self::Mul),
            "+" => Some(Self::Add),
            _ => None,
        }
    }

    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    op: Operation,
}

impl Problem {
    fn part1(&self) -> i64 {
        self.numbers
            .iter()
            .copied()
            .reduce(|acc, e| self.op.apply(acc, e))
            .unwrap()
    }
}

fn read_stdin() -> anyhow::Result<String> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    Ok(std::io::read_to_string(stdin_lock)?)
}

fn read_input_1(s: &str) -> anyhow::Result<Vec<Problem>> {
    let mut rows = vec![];
    let mut problems = vec![];
    for line in s.lines() {
        let mut row: Vec<i64> = vec![];
        let mut is_ops = false;
        for (i, field) in line.split_whitespace().enumerate() {
            if let Some(op) = Operation::from_str(field) {
                let numbers = rows.iter().map(|r: &Vec<i64>| r[i]).collect();
                problems.push(Problem { numbers, op });
                is_ops = true;
            } else {
                let number = field.parse::<i64>()?;
                row.push(number);
            }
        }
        if !is_ops {
            rows.push(row);
        }
    }
    Ok(problems)
}

fn read_input_2(s: &str) -> anyhow::Result<Vec<Problem>> {
    let lines = s.lines().filter(|l| !l.is_empty()).collect::<Vec<&str>>();
    let num_rows = lines.len();
    let num_cols = lines.iter().map(|l| l.trim().len()).max().unwrap();
    let mut separators = (0..num_cols)
        .filter(|i| lines.iter().all(|l| l.as_bytes()[*i] == b' '))
        .collect::<Vec<usize>>();
    separators.insert(0, 0);
    separators.push(num_cols + 1);
    let mut problems = vec![];
    for (mut lhs, rhs) in separators.iter().copied().tuple_windows() {
        if lhs != 0 {
            lhs += 1;
        }
        let Some(op) = Operation::from_str(&lines[num_rows - 1][lhs..lhs + 1]) else {
            panic!("invalid input at {}", &lines[num_rows - 1][lhs..lhs + 1])
        };
        let mut numbers = vec![];
        for col in lhs..rhs {
            let number = (0..(num_rows - 1))
                .filter_map(|row| {
                    let bytes = lines[row].as_bytes();
                    if bytes.len() > col {
                        Some(bytes[col] as char)
                    } else {
                        None
                    }
                })
                .collect::<String>();
            if !number.is_empty() {
                numbers.push(number.trim().parse()?);
            }
        }
        problems.push(Problem { numbers, op })
    }
    Ok(problems)
}

fn main() {
    tracing_subscriber::fmt::init();
    let input = read_stdin().unwrap();
    let input_1 = read_input_1(&input).unwrap();
    let part1: i64 = input_1.iter().map(|p| p.part1()).sum();
    println!("part 1: {}", part1);
    let input_2 = read_input_2(&input).unwrap();
    let part2: i64 = input_2.iter().map(|p| p.part1()).sum();
    println!("part 2: {}", part2);
}
