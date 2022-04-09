use crate::utils::*;
use crate::geometries::GeometryOps;

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
}

impl GeometryOps for GeoArc {    
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