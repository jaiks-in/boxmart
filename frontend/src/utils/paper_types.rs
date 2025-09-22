use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: i32,
    pub name: String,        // Brand name: "Cheema", "Khanna", "Bahl"
    pub variety: String,     // "Art Grade", "Duplex"
    pub gsm: i32,           // 150, 200, 250, etc.
    pub cost_per_kg: f64,   // Price per kg
}