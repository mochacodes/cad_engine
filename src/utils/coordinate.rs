#[derive(Debug, Clone)]
pub struct Coordinate {
    x: f64,
    y: f64
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate{
            x: x,
            y: y
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn translate_by(&self, x: f64, y: f64) -> Coordinate {
        Coordinate{
            x: self.x + x,
            y: self.y + y
        }
    }

    pub fn scale_by_factor(&self, factor: f64) -> Coordinate {
        Coordinate{
            x: self.x * factor,
            y: self.y * factor
        }
    }

    pub fn scale_x_y(&self, x: f64, y: f64) -> Coordinate {
        Coordinate{
            x: self.x * x,
            y: self.y * y
        }
    }
    
    pub fn max_values(c1: &Coordinate, c2: &Coordinate) -> Coordinate {
        Coordinate {
            x: c1.x.max(c2.x),
            y: c1.y.max(c2.y),
        }
    }

    pub fn min_values(c1: &Coordinate, c2: &Coordinate) -> Coordinate {
        Coordinate {
            x: c1.x.min(c2.x),
            y: c1.y.min(c2.y),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_by_tests() {
        let czero = Coordinate::new(0.0,0.0);
        let c2 = czero.translate_by(10.0, 50.0);

        let c3 = Coordinate::new(5.0,5.0);
        let c4 = c3.translate_by(8.0, 9.0);

        let c5 = c2.translate_by(-12.0, -25.0);

        let czeroed = czero.translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(5.0, 10.0)
                    .translate_by(-50.0, -100.0);

        assert_eq!(c2.x(), 10.0);
        assert_eq!(c2.y(), 50.0);

        assert_eq!(c4.x(), 13.0);
        assert_eq!(c4.y(), 14.0);

        assert_eq!(c5.x(), -2.0);
        assert_eq!(c5.y(), 25.0);

        assert_eq!(czero.x(), czeroed.x());
        assert_eq!(czero.y(), czeroed.y());
    }

    #[test]
    fn scale_by_factor_tests() {
        let c0 = Coordinate::new(0.0, 0.0).scale_by_factor(12.0);
        let c2 = Coordinate::new(10.0, 50.0).scale_by_factor(2.0);
        let c3 = Coordinate::new(5.0,5.0).scale_by_factor(10.0);
        let c4 = Coordinate::new(5.0,5.0).scale_by_factor(10.0).scale_by_factor(0.5);

        assert_eq!(c0.x(), 0.0);
        assert_eq!(c0.y(), 0.0);

        assert_eq!(c2.x(), 20.0);
        assert_eq!(c2.y(), 100.0);

        assert_eq!(c3.x(), 50.0);
        assert_eq!(c3.y(), 50.0);

        assert_eq!(c4.x(), 25.0);
        assert_eq!(c4.y(), 25.0);
    }

    #[test]
    fn scale_x_y_tests() {

        let c0 = Coordinate::new(0.0, 0.0).scale_x_y(12.0, 40.0);
        let c2 = Coordinate::new(10.0, 50.0).scale_x_y(2.0, 0.5);
        let c3 = Coordinate::new(5.0,5.0).scale_x_y(10.0, 0.2);
        let c4 = Coordinate::new(5.0,5.0).scale_x_y(10.0, 40.0).scale_x_y(0.5, 20.0);

        assert_eq!(c0.x(), 0.0);
        assert_eq!(c0.y(), 0.0);

        assert_eq!(c2.x(), 20.0);
        assert_eq!(c2.y(), 25.0);

        assert_eq!(c3.x(), 50.0);
        assert_eq!(c3.y(), 1.0);

        assert_eq!(c4.x(), 25.0);
        assert_eq!(c4.y(), 4000.0);
    }
}
