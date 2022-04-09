use crate::utils::*;
use crate::geometries::GeometryOps;

pub struct GeoPoint {
    coordinate: Coordinate,
    area: Area
}

impl GeoPoint {
    pub fn new(x: f64, y: f64) -> GeoPoint {
        let coord = Coordinate::new(x, y);
        GeoPoint {
            coordinate: coord.clone(),
            area: Area::new(coord.clone(), coord.clone())
        }
    }
}

impl GeometryOps for GeoPoint {    
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