#[derive(Copy,Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn distance(&self, other: &Point) -> f64{
        ((self.x-other.x).powf(2.0) +
        (self.y-other.y).powf(2.0)).sqrt()
    }

    pub fn middle(&self, other: &Point) -> Point{
        Point {
            x: (self.x+other.x)/2.0,
            y: (self.y+other.y)/2.0,
        }
    }
}

#[derive(Clone)]
/* Bezier */
pub struct Bezier{
    pub start: Point,
    pub control1: Point,
    pub control2: Point,
    pub end: Point,
}

impl<'a> Bezier{
    pub fn iter(&self) -> BezierIter{
        BezierIter { bezier: self, position: 0.0}
    } 
}

pub struct BezierIter<'a>{
    bezier: &'a Bezier,
    position: f64,
}

impl<'a> Iterator for BezierIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point>{
        if self.position > 1.0 {
            return None;
        }
        let x = 
            self.bezier.start.x * (1.0-self.position).powf(3.0) +
            3.0 * self.bezier.control1.x * self.position * (1.0-self.position).powf(2.0) + 
            3.0 * self.bezier.control2.x * self.position.powf(2.0) * (1.0-self.position) +
            self.bezier.end.x * self.position.powf(3.0);

        let y = 
            self.bezier.start.y * (1.0-self.position).powf(3.0) +
            3.0 * self.bezier.control1.y * self.position * (1.0-self.position).powf(2.0) + 
            3.0 * self.bezier.control2.y * self.position.powf(2.0) * (1.0-self.position) +
            self.bezier.end.y * self.position.powf(3.0);
        self.position += 0.01;
        Some(Point { x, y})
    }
}