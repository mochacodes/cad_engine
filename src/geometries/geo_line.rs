use crate::utils::*;
use crate::geometries::GeometryOps;

pub struct GeoLine {
    start: Coordinate,
    end: Coordinate,
    area: Area
}

impl GeoLine {
    pub fn new(start: Coordinate, end: Coordinate) -> GeoLine {
        GeoLine {
            start: start.clone(),
            end: end.clone(),
            area: Area::new(start, end)
        }
    }
}

impl GeometryOps for GeoLine {    
    fn area(&self) -> &Area {
        &self.area
    }
    
    fn translate_by(&self, by: Coordinate) {
        unimplemented!()
    }
    fn scale_by_factor(&self, factor: f64) {
        unimplemented!()
    }
    fn scale_x_y(&self, x: f64, y:f64) {
        unimplemented!()
    }
    fn rotate_by_angle(&self, angle: f64) {
        unimplemented!()
    }
    fn nearest_point_on_entity(&self, pt: Coordinate) -> Coordinate {
        unimplemented!()
    }
    fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate {
        unimplemented!()
    }

}