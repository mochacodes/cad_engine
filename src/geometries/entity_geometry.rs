use crate::geometries::*;

pub enum EntityGeometry {
    Line(GeoLine),
    Circle(GeoCircle),
    Point(GeoPoint),
    Arc(GeoArc)
}