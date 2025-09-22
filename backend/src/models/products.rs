use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBase {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,

    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,

    // Costing fields
    pub material_cost: f64,
    pub labour_cost: f64,
    pub margin: f64,
    pub final_cost: f64,

    pub has_logo: bool,
    pub is_urgent: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrugatedProduct {
    pub base: ProductBase,   // common fields

    pub ply: i32,
    pub roll_type: String,
    pub paper_gsm: i32,
    pub paper_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplexProduct {
    pub base: ProductBase,   // common fields

    pub has_printing: bool,
    pub printing_type: Option<String>,
    pub has_lamination: bool,
    pub lamination_type: Option<String>,
    pub paper_gsm: i32,
    pub paper_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitingCardProduct {
    pub base: ProductBase,   // common fields
    pub paper_type: String,
    pub paper_gsm: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductAll {
    Corrugated(CorrugatedProduct),
    Duplex(DuplexProduct),
    VisitingCard(VisitingCardProduct),
}
