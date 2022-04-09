use crate::utils::*;

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
    pub fn area(&self) -> &Area {
        &self.area
    }
    pub fn translate_by(&self, by: Coordinate) {
        unimplemented!()
    }
    pub fn scale_by_factor(&self, factor: f64) {
        unimplemented!()
    }
    pub fn scale_x_y(&self, x: f64, y:f64) {
        unimplemented!()
    }
    pub fn rotate_by_angle(&self, angle: f64) {
        unimplemented!()
    }
    pub fn nearest_point_on_entity(&self, pt: Coordinate) -> Coordinate {
        self.coordinate.clone()
    }
    pub fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate {
        self.coordinate.clone()
    }

}
