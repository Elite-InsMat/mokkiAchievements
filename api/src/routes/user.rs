use crate::{
    authentication::Claims,
    routes::user::user_structs::{
        AddAchievementRequest, LoginRequest, LoginResponse, RegisterRequest, UserResponse,
    },
    AppState, JWT_SECRET_KEY_NAME,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rocket::{
    http::Status,
    response::status::{BadRequest, Unauthorized},
    serde::json::Json,
    State,
};

use self::user_structs::User;

mod user_structs;

#[get("/get_user")]
pub async fn get_user(
    state: &State<AppState>,
    claims: Claims,
) -> Result<Json<UserResponse>, BadRequest<String>> {
    let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(claims.name)
        .fetch_one(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(UserResponse::from_user(user)))
}

#[put("/unlock_achievement", data = "<data>")]
pub async fn unlock_achievement(
    state: &State<AppState>,
    data: Json<AddAchievementRequest>,
    claims: Claims,
) -> Result<Status, BadRequest<String>> {
    let _: Result<Option<User>, sqlx::Error> = sqlx::query_as(
        "UPDATE users SET achievements = array_append(achievements, $1), timestamps = array_append(timestamps, $2) where username=$3",
    )
    .bind(data.id)
    .bind(data.timestamp)
    .bind(claims.name)
    .fetch_optional(&state.0)
    .await;

    Ok(Status::Ok)
}

#[post("/register", data = "<data>")]
pub async fn register(
    state: &State<AppState>,
    data: Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, BadRequest<String>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.0.password.as_bytes(), &salt)
        .map_err(|err| BadRequest(Some(err.to_string())))?
        .to_string();

    let user = Json(User {
        username: data.username.clone(),
        password_hash,
        achievements: vec![],
        timestamps: vec![],
    });
    let user:User = sqlx::query_as(
        "INSERT INTO users(username, password_hash, achievements, timestamps) VALUES ($1,$2,$3,$4) RETURNING id, username, password_hash, achievements, timestamps",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.achievements)
    .bind(&user.timestamps)
    .fetch_one(&state.0)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    let claim = Claims::from_name(&user.username);

    let response = LoginResponse {
        token: claim
            .into_token(&state.1.get(JWT_SECRET_KEY_NAME).unwrap())
            .map_err(|error| BadRequest(Some(error.1)))?,
        username: user.username,
    };
    Ok(Json(response))
}

#[post("/login", data = "<login>")]
pub async fn login(
    state: &State<AppState>,
    login: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Unauthorized<String>> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(&login.username)
        .fetch_one(&state.0)
        .await
        .map_err(|_e| Unauthorized(Some("No such user".to_string())))?;

    let argon2 = Argon2::default();

    if argon2
        .verify_password(
            login.password.as_bytes(),
            &PasswordHash::new(&user.password_hash).unwrap(),
        )
        .is_ok()
    {
        let claim = Claims::from_name(&login.username);
        let response = LoginResponse {
            token: claim
                .into_token(&state.1.get(JWT_SECRET_KEY_NAME).unwrap())
                .map_err(|error| Unauthorized(Some(error.1)))?,
            username: login.username.clone(),
        };

        Ok(Json(response))
    } else {
        Err(Unauthorized(Some("Incorrect password".to_string())))
    }
}
