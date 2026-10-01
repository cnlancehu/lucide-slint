use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Icon {
    pub name_pascal: String,
    pub paths: Vec<Path>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub commands: String,
    pub has_fill: bool,
}
