pub trait Vistor {

    fn visit_line(&self, line: Line);
    fn visit_circle(&self, circle: Circle);
    fn visit_arc(&self, arc: Arc);
    fn visit_point(&self, point: Point);
}

pub struct Line;
pub struct Circle;
pub struct Arc;
pub struct Point;
