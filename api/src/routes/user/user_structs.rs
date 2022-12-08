use serde::{Deserialize, Serialize};
use sqlx::FromRow;

type TimeStamp = i64;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub achievements: Vec<i32>,
    pub timestamps: Vec<TimeStamp>,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct AddAchievementRequest {
    pub id: i32,
    pub timestamp: TimeStamp,
}
#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub achievements: Vec<i32>,
    pub timestamps: Vec<TimeStamp>,
}
impl UserResponse {
    pub fn from_user(user: User) -> Self {
        UserResponse {
            username: user.username,
            achievements: user.achievements,
            timestamps: user.timestamps,
        }
    }
}
