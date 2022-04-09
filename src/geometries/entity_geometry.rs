use crate::geometries::*;
use crate::utils::*;

pub enum EntityGeometry {
    Line(GeoLine),
    Circle(GeoCircle),
    Point(GeoPoint),
    Arc(GeoArc)
}

impl EntityGeometry {
    pub fn translate_by(&self, by: Coordinate) {
        match self {
            Self::Line(geometry) => geometry.translate_by(by),
            Self::Arc(geometry) => geometry.translate_by(by),
            Self::Circle(geometry) => geometry.translate_by(by),
            Self::Point(geometry) => geometry.translate_by(by),
        }
    }
    pub fn scale_by_factor(&self, factor: f64) {
        match self {
            Self::Line(geometry) => geometry.scale_by_factor(factor),
            Self::Arc(geometry) => geometry.scale_by_factor(factor),
            Self::Circle(geometry) => geometry.scale_by_factor(factor),
            Self::Point(geometry) => geometry.scale_by_factor(factor),
        }
    }
    pub fn scale_x_y(&self, x: f64, y:f64) {
        match self {
            Self::Line(geometry) => geometry.scale_x_y(x, y),
            Self::Arc(geometry) => geometry.scale_x_y(x, y),
            Self::Circle(geometry) => geometry.scale_x_y(x, y),
            Self::Point(geometry) => geometry.scale_x_y(x, y),
        }
    }
    pub fn rotate_by_angle(&self, angle: f64) {
        unimplemented!()
    }
    pub fn area(&self) -> &Area {
        match self {
            Self::Line(geometry) => geometry.area(),
            Self::Arc(geometry) => geometry.area(),
            Self::Circle(geometry) => geometry.area(),
            Self::Point(geometry) => geometry.area(),
        }
    }
    pub fn nearest_point_on_entity(&self, pt: Coordinate) -> Coordinate {
        match self {
            Self::Line(geometry) => geometry.nearest_point_on_entity(pt),
            Self::Arc(geometry) => geometry.nearest_point_on_entity(pt),
            Self::Circle(geometry) => geometry.nearest_point_on_entity(pt),
            Self::Point(geometry) => geometry.nearest_point_on_entity(pt),
        }
    }
    pub fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate {
        match self {
            Self::Line(geometry) => geometry.nearest_point_on_path(pt),
            Self::Arc(geometry) => geometry.nearest_point_on_path(pt),
            Self::Circle(geometry) => geometry.nearest_point_on_path(pt),
            Self::Point(geometry) => geometry.nearest_point_on_path(pt),
        }
    }
}