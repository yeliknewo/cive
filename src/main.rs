extern crate rand;

use rand::Rng;

type Coord = i16;
type CoordF = f64;

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Point {
    x: Coord,
    y: Coord,
}

impl Point {
    #[inline]
    pub fn new(x: Coord, y: Coord) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    #[inline]
    fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> Line {
        Line::new(Point::new(x0, y0), Point::new(x1, y1))
    }

    #[inline]
    fn new(a: Point, b: Point) -> Line {
        Line {
            a: a,
            b: b,
        }
    }

    #[inline]
    fn collides_static(l1: Line, l2: Line) -> bool {
        Line::cross_signum_static(l1, l2.a) == Line::cross_signum_static(l1, l2.b)
    }

    #[inline]
    fn cross_signum_static(l: Line, p: Point) -> Coord {
        (l.a.x - l.b.x) * (p.y - l.a.y) - (l.a.y - l.b.y) * (p.y - l.a.x).signum()
    }

    #[inline]
    fn length_static(l: Line) -> Coord {
        (((l.a.x - l.b.x).pow(2) + (l.a.y - l.b.y).pow(2)) as CoordF).sqrt() as Coord
    }

    #[inline]
    fn mid_point_static(l: Line) -> Point {
        Point::new(((l.a.x + l.b.x) as CoordF / 2.0) as Coord, ((l.a.y + l.b.y) as CoordF / 2.0) as Coord)
    }

    #[inline]
    fn collides(&self, other: Line) -> bool {
        Line::collides_static(*self, other)
    }

    #[inline]
    fn length(&self) -> Coord {
        Line::length_static(*self)
    }

    #[inline]
    fn mid_point(&self) -> Point {
        Line::mid_point_static(*self)
    }
}

#[derive(Copy, Clone)]
struct Rect {
    corners: Line,
}

impl Rect {
    #[inline]
    fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> Rect {
        Rect::new(Line::new_from_coords(x0, y0, x1, y1))
    }

    #[inline]
    fn new_from_points(p0: Point, p1: Point) -> Rect {
        Rect::new(Line::new(p0, p1))
    }

    #[inline]
    fn new(corners: Line) -> Rect {
        Rect {
            corners: corners,
        }
    }

    #[inline]
    fn get_x0(&self) -> Coord {
        self.corners.a.x
    }

    #[inline]
    fn get_x1(&self) -> Coord {
        self.corners.b.x
    }

    #[inline]
    fn get_y0(&self) -> Coord {
        self.corners.a.y
    }

    #[inline]
    fn get_y1(&self) -> Coord {
        self.corners.b.y
    }
}

struct Polygon {
    lines: Vec<Line>,
}

struct Voronoi {
    polygons: Vec<Polygon>,
}

impl Voronoi {
    fn new(point_count: u64, point_space: Rect) -> Voronoi {

        let points = Voronoi::gen_points(point_count);

        let lines = Voronoi::gen_lines_initial(points);

        let lines = Voronoi::gen_lines_midpoint(lines);

        let lines = Voronoi::prune_lines(lines);

        for line in lines {
            println!("{} {} {} {}", line.a.x, line.a.y, line.b.x, line.b.y);
        }

        // let polygons = Voronoi::gen_polygons(lines);

        Voronoi {
            polygons: vec!(

            ),
        }
    }

    fn gen_points(point_count: u64) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut points = vec!();

        for _ in 0..point_count {
            points.push(Point::new((rng.gen::<Coord>() as CoordF).log(2.0) as Coord, (rng.gen::<Coord>() as CoordF).log(2.0) as Coord));
        }

        points
    }

    fn gen_lines_initial(points: Vec<Point>) -> Vec<Line> {
        let mut lines = vec!();

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                lines.push(Line::new(*points.get(i).expect("i was none"), *points.get(j).expect("j was none")));
            }
        }

        lines
    }

    fn gen_lines_midpoint(lines: Vec<Line>) -> Vec<Line> {
        let mut mids = vec!();

        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                mids.push(Line::new(lines.get(i).expect("i was none").mid_point(), lines.get(j).expect("j was none").mid_point()));
            }
        }

        mids
    }

    fn prune_lines(mut lines: Vec<Line>) -> Vec<Line> {
        let mut pruned = vec!();

        lines.sort_by(|a, b| b.length().cmp(&a.length()));

        for i in 0..lines.len() {
            let line_i = lines.get(i).expect("i was none");

            let mut keep = true;
            for j in (i + 1)..lines.len() {
                if line_i.collides(*lines.get(j).expect("j was none")) {
                    keep = false;
                    break;
                }
            }
            if keep {
                pruned.push(*line_i);
            }
        }

        pruned
    }
}

fn main() {
    Voronoi::new(10, Rect::new_from_coords(-10, -10, 10, 10));
}
