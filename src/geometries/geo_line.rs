use crate::utils::*;

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