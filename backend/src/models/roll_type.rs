use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RollType {
    pub id: i32,
    pub roll_type: String,           
    pub cost: String,        
           
}
