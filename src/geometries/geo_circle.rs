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
}