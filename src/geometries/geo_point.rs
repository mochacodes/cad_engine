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
}