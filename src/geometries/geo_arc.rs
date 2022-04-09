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
}