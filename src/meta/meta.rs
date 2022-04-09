pub struct Meta {
    layer: String,
    line_width: String
}

impl Meta {
    pub fn default() -> Meta {
        Meta {
            layer: "0".to_string(),
            line_width: "1".to_string()
        }
    }

    pub fn layer_name(&self) -> &str {
        &self.layer
    }

    pub fn line_width(&self) -> &str {
        &self.line_width
    }
}

pub struct MetaBuilder {
    meta: Meta
}

impl MetaBuilder {
    pub fn new() -> MetaBuilder {
        MetaBuilder{
            meta: Meta {
                layer: "0".to_string(),
                line_width: "0".to_string()
            }
        }
    }
    pub fn with_layer(&mut self, layer_name: String) -> &mut MetaBuilder {
        self.meta.layer = layer_name;
        self
    }

    pub fn with_width(&mut self, width: String) -> &mut MetaBuilder {
        self.meta.line_width = width;
        self
    }

    pub fn build(self) -> Meta {
        self.meta
    }
}