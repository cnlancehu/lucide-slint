use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Icon {
    pub name_pascal: String,
    pub url: String,
    pub deprecated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconMetadata {
    #[serde(default)]
    pub deprecated: bool,
}
