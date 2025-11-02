use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Icon {
    pub name_pascal: String,
    pub url: String,
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
    pub x_scale: f32,
    pub y_scale: f32,
    pub width_scale: f32,
    pub height_scale: f32,
    pub commands: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconMetadata {
    #[serde(default)]
    pub deprecated: bool,
}
