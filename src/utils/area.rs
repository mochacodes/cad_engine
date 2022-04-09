use crate::utils::Coordinate;

pub struct Area {
    min_point: Coordinate,
    max_point: Coordinate
}

impl Area {
    pub fn new(pt1: Coordinate, pt2: Coordinate) -> Area {
        Area{
            min_point: Coordinate::min_values(&pt1, &pt2),
            max_point: Coordinate::max_values(&pt1, &pt2)
        }
    }

    pub fn intersects_area(&self, area2: &Area) -> bool {
        !(area2.max_point().x() < self.min_point().x() ||
        self.max_point().x() < area2.min_point().x() ||
        area2.max_point().y() < self.min_point().y() ||
        self.max_point().y() < area2.min_point().y())
    }

    pub fn contains_coordinate(&self, pt: &Coordinate) -> bool {
        self.min_point().x() < pt.x() && pt.x() < self.max_point().x() &&
        self.min_point().y() < pt.y() && pt.y() < self.max_point().y() 
    }

    pub fn contains_area(&self, pt: &Area) -> bool {
        self.contains_coordinate(pt.min_point()) && self.contains_coordinate(pt.max_point())
    }

    pub fn get_union(&self, area2: &Area) -> Area {
        Area {
            min_point: Coordinate::min_values(&self.min_point, area2.min_point()),
            max_point: Coordinate::min_values(&self.max_point, area2.max_point())
        }
    }

    pub fn height(&self) -> f64 {
        self.max_point().y() - self.min_point().y()
    }

    pub fn width(&self) -> f64 {
        self.max_point().x() - self.min_point().x()
    }

    pub fn min_point(&self) -> &Coordinate {
        &self.min_point
    }

    pub fn max_point(&self) -> &Coordinate {
        &self.max_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_area_test() {
        let area1 = Area::new(Coordinate::new(0.0,0.0), Coordinate::new(10.0,10.0));
        let area2 = Area::new(Coordinate::new(20.0,20.0), Coordinate::new(30.0,30.0));
        let area3 = Area::new(Coordinate::new(5.0,5.0), Coordinate::new(10.0,10.0));
        let area4 = Area::new(Coordinate::new(5.0,5.0), Coordinate::new(8.0,8.0));

        assert_eq!(area1.intersects_area(&area2), false);
        assert_eq!(area2.intersects_area(&area1), false);

        assert_eq!(area3.intersects_area(&area2), false);
        assert_eq!(area2.intersects_area(&area3), false);

        assert_eq!(area4.intersects_area(&area2), false);
        assert_eq!(area2.intersects_area(&area4), false);

        assert_eq!(area1.intersects_area(&area3), true);
        assert_eq!(area3.intersects_area(&area1), true);

        assert_eq!(area1.intersects_area(&area4), true);
        assert_eq!(area4.intersects_area(&area1), true);

        assert_eq!(area3.intersects_area(&area4), true);
        assert_eq!(area4.intersects_area(&area3), true);
    }

    #[test]
    fn width_height_tests() {
        let area1 = Area::new(Coordinate::new(0.0,0.0), Coordinate::new(10.0,10.0));
        let area2 = Area::new(Coordinate::new(30.0,30.0), Coordinate::new(20.0,20.0));
        let area3 = Area::new(Coordinate::new(5.0,5.0), Coordinate::new(10.0,10.0));
        let area4 = Area::new(Coordinate::new(5.0,5.0), Coordinate::new(8.0,8.0));

        assert_eq!(area1.width(), 10.0);
        assert_eq!(area2.width(), 10.0);
        assert_eq!(area3.width(), 5.0);
        assert_eq!(area4.width(), 3.0);

        assert_eq!(area1.height(), 10.0);
        assert_eq!(area2.height(), 10.0);
        assert_eq!(area3.height(), 5.0);
        assert_eq!(area4.height(), 3.0);
        
    }
}
