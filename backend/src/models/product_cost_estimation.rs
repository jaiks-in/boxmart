use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostRequest {
    pub product_id: i32,
    pub quantity: i32,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub paper_gsm:Option<i32>
    pub paper_type:Option<String>,
    pub roll_type:Option<String>,
    pub ply:Option<i32>,
    pub has_logo: bool,
    pub is_urgent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostResponse {
    pub product_id: i32,
    pub quantity: i32,
    pub lamination_cost: f64,
    pub printing_cost: f64,
    pub urgency_fee: f64,
    pub margin:Option<String>,
    pub paper_cost:f64,
    pub roll_cost:f64,
    pub labour_manufacturing_cost:f64,
    pub total_cost:f32,
    pub estimated_days: i32,
}