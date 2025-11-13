use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Icon {
    pub name_pascal: String,
    pub deprecated: bool,
    pub svg: Svg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Svg {
    pub size: f32,
    pub paths: Vec<Path>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub viewbox_x: f32,
    pub viewbox_y: f32,
    pub viewbox_width: f32,
    pub viewbox_height: f32,
    pub commands: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconMetadata {
    #[serde(default)]
    pub deprecated: bool,
}
