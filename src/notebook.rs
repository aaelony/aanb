

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Notebook {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
pub struct Cell {
    #[serde(rename = "cell_type")]
    pub cell_type: String,

    pub source: Vec<String>,

    #[serde(default)]
    pub metadata: serde_json::Value,

    #[serde(default)]
    pub outputs: Vec<serde_json::Value>,
}






