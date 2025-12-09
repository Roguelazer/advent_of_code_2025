use aoclib::Point;
use itertools::Itertools;
use tap::Pipe;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
struct TileId(usize);

fn read_input() -> anyhow::Result<Vec<(TileId, Point)>> {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let s = std::io::read_to_string(stdin_lock)?;
    s.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let (x, y) = line.split_once(',')?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;
            let i = TileId(i);
            Some((i, Point::new(x, y)))
        })
        .collect::<Vec<(TileId, Point)>>()
        .pipe(Ok)
}

fn part1(points: &[(TileId, Point)]) -> i64 {
    points
        .iter()
        .cartesian_product(points.iter())
        .filter_map(|((lid, lp), (rid, rp))| {
            if lid >= rid {
                return None;
            }
            let area = ((lp.x - rp.x).abs() + 1) * ((lp.y - rp.y).abs() + 1);
            tracing::trace!("{:?} ({:?}) {:?} ({:?}) -> {}", lid, lp, rid, rp, area);
            Some(area)
        })
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    lhs: Point,
    rhs: Point,
}

impl Edge {
    fn new(p1: Point, p2: Point) -> Self {
        if p1.y == p2.y {
            if p1.x < p2.x {
                Edge { lhs: p1, rhs: p2 }
            } else {
                Edge { lhs: p2, rhs: p1 }
            }
        } else if p1.x == p2.x {
            if p1.y < p2.y {
                Edge { lhs: p1, rhs: p2 }
            } else {
                Edge { lhs: p2, rhs: p1 }
            }
        } else {
            panic!("not a straight line!");
        }
    }

    fn vertical_x(&self) -> Option<i64> {
        if self.is_vertical() {
            Some(self.lhs.x)
        } else {
            None
        }
    }

    fn horizontal_y(&self) -> Option<i64> {
        if self.is_horizontal() {
            Some(self.lhs.y)
        } else {
            None
        }
    }

    fn is_horizontal(&self) -> bool {
        self.lhs.y == self.rhs.y
    }

    fn is_vertical(&self) -> bool {
        self.lhs.x == self.rhs.x
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new(p1: Point, p2: Point) -> Self {
        let tlx = std::cmp::min(p1.x, p2.x);
        let tly = std::cmp::min(p1.y, p2.y);
        let brx = std::cmp::max(p1.x, p2.x);
        let bry = std::cmp::max(p1.y, p2.y);
        Self {
            top_left: Point::new(tlx, tly),
            bottom_right: Point::new(brx, bry),
        }
    }

    fn width(&self) -> i64 {
        self.bottom_right.x - self.top_left.x + 1
    }

    fn height(&self) -> i64 {
        self.bottom_right.y - self.top_left.y + 1
    }

    fn area(&self) -> i64 {
        self.width() * self.height()
    }

    fn is_crossed_by(&self, edge: &Edge) -> bool {
        if let Some(x) = edge.vertical_x()
            && x > self.top_left.x
            && x < self.bottom_right.x
            && self.top_left.y.max(edge.lhs.y) < self.bottom_right.y.min(edge.rhs.y)
        {
            return true;
        }
        if let Some(y) = edge.horizontal_y()
            && y > self.top_left.y
            && y < self.bottom_right.y
            && self.top_left.x.max(edge.lhs.x) < self.bottom_right.x.min(edge.rhs.x)
        {
            return true;
        }
        false
    }
}

fn part2(points: &[(TileId, Point)]) -> i64 {
    let edges = points
        .iter()
        .circular_tuple_windows()
        .map(|(l, r)| {
            let lhs = l.1;
            let rhs = r.1;
            Edge::new(lhs, rhs)
        })
        .collect::<Vec<Edge>>();
    points
        .iter()
        .cartesian_product(points.iter())
        .filter_map(|((lid, lp), (rid, rp))| {
            if lid >= rid {
                return None;
            }
            let rect = Rectangle::new(*lp, *rp);
            tracing::trace!(
                "analyzing {:?}, {:?} : {:?},{:?} -> {:?}",
                lid,
                rid,
                lp,
                rp,
                rect
            );
            let bisected = edges.iter().any(|edge| {
                if rect.is_crossed_by(edge) {
                    tracing::trace!("{:?} is crossed by {:?}", rect, edge);
                    true
                } else {
                    false
                }
            });
            if bisected {
                tracing::trace!("invalid!");
                return None;
            }
            tracing::trace!(area = rect.area(), "valid!");
            Some(rect.area())
        })
        .max()
        .unwrap()
}

fn main() {
    tracing_subscriber::fmt::init();
    let input = read_input().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
