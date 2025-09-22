use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Brand {
    pub id: i32,
    pub name: String,           // Brand name: Cheema, Khanna, Bahl...
    pub variety: String,        // 'Duplex' or 'Art Grade'
    pub gsm: i32,               // 150, 200, 250, 300, 350
    pub cost_per_kg: f64,       // Cost per kg in Rs
}
