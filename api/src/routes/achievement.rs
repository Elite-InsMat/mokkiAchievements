use rocket::{http::Status, response::status::BadRequest, serde::json::Json, State};

use crate::{routes::achievement::achievement_structs::Achievement, AppState};

pub mod achievement_structs;

#[get("/get_achievements")]
pub async fn get_achievements(
    state: &State<AppState>,
) -> Result<Json<Vec<Achievement>>, BadRequest<String>> {
    let achievements: Vec<Achievement> = sqlx::query_as("SELECT * FROM achievements")
        .fetch_all(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(achievements))
}

#[post("/add_achievement", data = "<data>")]
pub async fn add_achievement(
    state: &State<AppState>,
    data: Json<Achievement>,
) -> Result<Status, BadRequest<String>> {
    let _user: Achievement = sqlx::query_as(
        "INSERT INTO achievements(name, description, image) VALUES ($1,$2,$3) RETURNING name, description, image",
    )
    .bind(&data.name)
    .bind(&data.description)
    .bind(&data.image)
    .fetch_one(&state.0)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}
