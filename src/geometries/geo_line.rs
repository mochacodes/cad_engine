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
}