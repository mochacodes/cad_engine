use crate::utils::*;

pub struct GeoCircle {
    center: Coordinate,
    radius: f64,
    area: Area
}

impl GeoCircle {
    pub fn new(center: Coordinate, radius: f64) -> GeoCircle {
        GeoCircle {
            center: center.clone(),
            radius: radius,
            area: Area::new(Coordinate::new(center.x() + radius, center.y() + radius), Coordinate::new(center.x() - radius, center.y() - radius))
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
        unimplemented!()
    }
    pub fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate {
        unimplemented!()
    }
}