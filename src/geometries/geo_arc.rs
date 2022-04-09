use crate::utils::*;

pub struct GeoArc {
    center: Coordinate,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    area: Area
}

impl GeoArc {
    pub fn new(center: Coordinate, radius: f64, start_angle: f64, end_angle: f64) -> GeoArc {
        GeoArc {
            center: center.clone(),
            radius: radius,
            start_angle: start_angle,
            end_angle: end_angle,
            // TODO Optimize this area. an arc should not have area of a complete circle.
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