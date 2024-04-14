use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification{
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String,
}