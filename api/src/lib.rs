#[macro_use]
extern crate rocket;

use cors::Cors;
use routes::{
    achievement::{add_achievement, get_achievements},
    user::{get_user, login, register, unlock_achievement},
};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

mod authentication;
mod cors;
mod routes;

pub const JWT_SECRET_KEY_NAME: &str = "JWT_SECRET";

pub struct AppState(PgPool, SecretStore);

#[get("/")]
fn index() -> &'static str {
    "Hello, mökki!"
}

#[shuttle_service::main]
async fn rocket(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleRocket {
    let state = AppState(pool, secret_store);

    Ok(rocket::build()
        .manage(state)
        .attach(Cors)
        .mount("/", routes![index])
        .mount(
            "/user",
            routes![get_user, login, register, unlock_achievement],
        )
        .mount("/achievement", routes![get_achievements, add_achievement]))
}
