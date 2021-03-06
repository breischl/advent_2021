use crate::array_grid::ArrayGrid;

pub fn run(input: String) -> Result<String, String> {
    let mut grid: ArrayGrid<VentSquare> = ArrayGrid::create_square(1000);

    input
        .lines()
        .map(parse_line)
        .flat_map(|ls| ls.get_all_points())
        .for_each(|point| {
            grid.get_mut(point.x, point.y).num_vents += 1;
        });

    let overlaps = grid.iter().filter(|vs| vs.num_vents > 1).count();

    Ok(format!("overlaps: {}", overlaps))
}

fn parse_line(line: &str) -> LineSegment {
    let list: Vec<&str> = line.split(" -> ").collect();
    if let [start_pair, end_pair] = &list[..] {
        LineSegment {
            start: parse_coordinate(start_pair),
            end: parse_coordinate(end_pair),
        }
    } else {
        panic!("Unable to parse {} as a LineSegment", line);
    }
}

fn parse_coordinate(raw_coord: &str) -> Coordinate {
    let list: Vec<&str> = raw_coord.split(',').collect();
    if let [raw_x, raw_y] = &list[..] {
        Coordinate {
            x: raw_x.parse::<usize>().unwrap(),
            y: raw_y.parse::<usize>().unwrap(),
        }
    } else {
        panic!("Unable to parse {} as a Coordinate", raw_coord);
    }
}

#[derive(Clone, Default, Debug)]
struct VentSquare {
    num_vents: u16,
}

impl std::fmt::Display for VentSquare {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.num_vents)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "(x:{}, y:{})", self.x, self.y)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct LineSegment {
    start: Coordinate,
    end: Coordinate,
}

impl LineSegment {
    fn get_all_points(&self) -> Vec<Coordinate> {
        if self.start.x == self.end.x {
            safe_range_inclusive(self.start.y, self.end.y)
                .map(|y| Coordinate { x: self.start.x, y })
                .collect()
        } else if self.start.y == self.end.y {
            safe_range_inclusive(self.start.x, self.end.x)
                .map(|x| Coordinate { x, y: self.start.y })
                .collect()
        } else {
            safe_range_inclusive(self.start.x, self.end.x)
                .zip(safe_range_inclusive(self.start.y, self.end.y))
                .map(|(x, y)| Coordinate { x, y })
                .collect()
        }
    }
}

fn safe_range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

impl std::fmt::Display for LineSegment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{},{} -> {},{}",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn linesegment_get_all_points_vertical_works() {
        let ls = LineSegment {
            start: Coordinate { x: 1, y: 1 },
            end: Coordinate { x: 1, y: 3 },
        };
        let points = ls.get_all_points();
        points.iter().for_each(|p| println!("{:?}", p));

        assert_eq!(points[0], Coordinate { x: 1, y: 1 });
        assert_eq!(points[1], Coordinate { x: 1, y: 2 });
        assert_eq!(points[2], Coordinate { x: 1, y: 3 });
    }

    #[test]
    pub fn linesegment_get_all_points_horizontal_works() {
        let ls = LineSegment {
            start: Coordinate { x: 1, y: 1 },
            end: Coordinate { x: 4, y: 1 },
        };
        let points = ls.get_all_points();
        points.iter().for_each(|p| println!("{:?}", p));

        assert_eq!(points[0], Coordinate { x: 1, y: 1 });
        assert_eq!(points[1], Coordinate { x: 2, y: 1 });
        assert_eq!(points[2], Coordinate { x: 3, y: 1 });
        assert_eq!(points[3], Coordinate { x: 4, y: 1 });
    }

    #[test]
    pub fn linesegment_get_all_points_horizontal_works_desc() {
        let ls = LineSegment {
            start: Coordinate { x: 4, y: 1 },
            end: Coordinate { x: 1, y: 1 },
        };
        let points = ls.get_all_points();
        points.iter().for_each(|p| println!("{:?}", p));

        assert_eq!(points[0], Coordinate { x: 4, y: 1 });
        assert_eq!(points[1], Coordinate { x: 3, y: 1 });
        assert_eq!(points[2], Coordinate { x: 2, y: 1 });
        assert_eq!(points[3], Coordinate { x: 1, y: 1 });
    }

    #[test]
    pub fn linesegment_get_all_points_diagonal_works() {
        let ls = LineSegment {
            start: Coordinate { x: 1, y: 3 },
            end: Coordinate { x: 3, y: 1 },
        };
        let points = ls.get_all_points();
        points.iter().for_each(|p| println!("{:?}", p));

        assert_eq!(points[0], Coordinate { x: 1, y: 3 });
        assert_eq!(points[1], Coordinate { x: 2, y: 2 });
        assert_eq!(points[2], Coordinate { x: 3, y: 1 });
    }

    #[test]
    pub fn safe_range_inclusivee_asc() {
        let r: Vec<usize> = safe_range_inclusive(1, 10).collect();
        assert_eq!(r.first().unwrap(), &1);
        assert_eq!(r.last().unwrap(), &10);
    }

    #[test]
    pub fn safe_range_inclusive_desc() {
        let r: Vec<usize> = safe_range_inclusive(10, 1).collect();
        assert_eq!(r.first().unwrap(), &10);
        assert_eq!(r.last().unwrap(), &1);
    }
}
