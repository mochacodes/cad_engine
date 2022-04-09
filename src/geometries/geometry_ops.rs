use crate::utils::*;

pub trait GeometryOps {
    fn translate_by(&self, by: Coordinate);
    fn scale_by_factor(&self, factor: f64);
    fn scale_x_y(&self, x: f64, y:f64);
    fn rotate_by_angle(&self, angle: f64);
    fn area(&self) -> &Area;
    fn nearest_point_on_entity(&self, pt: Coordinate) -> Coordinate;
    fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate;
}