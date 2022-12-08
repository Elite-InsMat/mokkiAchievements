use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
}
