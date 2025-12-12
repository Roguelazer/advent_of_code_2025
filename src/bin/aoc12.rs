use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of, space1};
use nom::combinator::map;
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{separated_pair, terminated};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Cell {
    Full,
    #[default]
    Empty,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Full,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Present {
    #[allow(unused)]
    index: usize,
    shape: [[Cell; 3]; 3],
}

impl Present {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(nom::character::complete::u32, tag(":\n"), Self::parse_shape),
            |(index, shape)| Present {
                index: index as usize,
                shape,
            },
        )(s)
    }

    fn parse_shape(s: &str) -> IResult<&str, [[Cell; 3]; 3]> {
        map(
            many_m_n(3, 3, terminated(many_m_n(3, 3, one_of("#.")), newline)),
            |lines| {
                lines
                    .into_iter()
                    .map(|l| {
                        l.into_iter()
                            .map(Cell::from_char)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<[Cell; 3]>>()
                    .try_into()
                    .unwrap()
            },
        )(s)
    }

    fn covered_cells(&self) -> u32 {
        self.shape
            .iter()
            .map(|l| l.iter().filter(|c| **c == Cell::Full).count() as u32)
            .sum()
    }
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    quantities: Vec<u32>,
}

impl Region {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_pair(
                    nom::character::complete::u32,
                    tag("x"),
                    nom::character::complete::u32,
                ),
                tag(": "),
                separated_list1(space1, nom::character::complete::u32),
            ),
            |((width, height), quantities)| Region {
                width,
                height,
                quantities,
            },
        )(s)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_possibly_fit(&self, presents: &[Present]) -> bool {
        // each present needs some number of cells, so we can immediately discard
        // those that won't fit on the board
        let needed_area = self
            .quantities
            .iter()
            .zip(presents)
            .filter(|(q, _)| **q > 0)
            .map(|(q, present)| present.covered_cells() * q)
            .sum();
        self.area() >= needed_area
    }

    fn can_definitely_fit(&self) -> bool {
        // each present is a 3x3 grid; if our area is big enough to put them
        // all without any overlapping, then we don't need to try any more
        let total_presents = self.quantities.iter().sum();
        (self.width / 3) * (self.height / 3) >= total_presents
    }

    fn fits(&self, presents: &[Present]) -> bool {
        if !self.can_possibly_fit(presents) {
            return false;
        }
        if self.can_definitely_fit() {
            return true;
        }
        // how would you even solve this? there are hundreds of presents
        // and thousands of cells, and I suspect an actual solution would have
        // something like O(2^(P+C)) complexity...
        todo!()
    }
}

#[derive(Debug)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

impl Problem {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_list1(newline, Present::parse),
                newline,
                separated_list1(newline, Region::parse),
            ),
            |(presents, regions)| Problem { presents, regions },
        )(s)
    }
}

fn read_input() -> anyhow::Result<Problem> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let s = std::io::read_to_string(stdin_lock)?;
    let (remainder, p) =
        Problem::parse(&s).map_err(|e| anyhow::anyhow!("failed to parse: {:?}", e))?;
    if !remainder.trim().is_empty() {
        anyhow::bail!("unhandled parse input: {}", remainder);
    }
    Ok(p)
}

fn main() {
    tracing_subscriber::fmt::init();
    let problem = read_input().unwrap();
    let part1 = problem
        .regions
        .iter()
        .filter(|r| r.fits(&problem.presents))
        .count();
    println!("part 1: {}/{}", part1, problem.regions.len());
}
