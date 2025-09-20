use serde::{Serialize, Deserialize};

/// Form validation ke liye payload
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckPayload {
    pub id: i32,                       // agar optional banana ho to Option<i32>
    pub name: String,
    pub username: String,
    pub org_name: Option<String>,      // company name optional hai
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub user_type:Option<String>
}

/// Backend ko bhejne ke liye payload
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignupPayload {
    pub name: String,
    pub username: String,
    pub org_name: Option<String>,
    pub email: String,
    pub user_type:Option<String>,
    pub password: String,
}

/// Agar backend se koi response aata hai
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub message: String,
    pub user_id: Option<i32>, // agar backend user id return karta hai
}

#[derive(Deserialize,Serialize)]
pub struct LoginPayload{
    pub email:String,
    pub password:String
}