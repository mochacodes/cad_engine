use crate::utils::*;
use crate::geometries::EntityGeometry;
use crate::meta::Meta;

pub struct BaseEntity {
    geometry: EntityGeometry,
    meta: Meta,
    id: usize,
}

impl BaseEntity {

    pub fn with_default_meta(geometry: EntityGeometry) -> BaseEntity {
        BaseEntity {
            geometry: geometry,
            meta: Meta::default(),
            id: 0
        }
    }

    pub fn with_meta(geometry: EntityGeometry, meta: Meta) -> BaseEntity {
        BaseEntity {
            geometry: geometry,
            meta: meta,
            id: 0
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn translate_by(&self, by: Coordinate) {
        self.geometry.translate_by(by)
    }
    pub fn scale_by_factor(&self, factor: f64) {
        self.geometry.scale_by_factor(factor)        
    }
    pub fn scale_x_y(&self, x: f64, y:f64) {
        self.geometry.scale_x_y(x, y)
    }
    pub fn rotate_by_angle(&self, angle: f64) {
        self.geometry.rotate_by_angle(angle)        
    }
    pub fn area(&self) -> &Area {
        self.geometry.area()
    }
    pub fn nearest_point_on_entity(&self, pt: Coordinate) -> Coordinate {
        self.geometry.nearest_point_on_entity(pt)
    }
    pub fn nearest_point_on_path(&self, pt: Coordinate) -> Coordinate {
        self.geometry.nearest_point_on_entity(pt)        
    }
    pub fn layer(&self) -> &str {
        self.meta.layer_name()
    }    
    pub fn line_width(&self) -> &str {
        self.meta.line_width()
    }

}