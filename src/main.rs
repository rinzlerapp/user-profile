#[macro_use] extern crate rocket;
mod config;
mod types;

use mongodb::options::{ClientOptions, UpdateOptions};
use mongodb::{Client, bson};
use rocket::serde::json::Json;
use rocket::State;

struct App {
    mongo: mongodb::Client,
    profiles: mongodb::Collection<types::Profile>,
}

#[get("/health")]
fn health() -> &'static str {
    "{\"status\": \"ok\"}"
}

#[get("/profiles/<id>", format = "application/json")]
async fn get_profile(app: &State<App> ,id: String) -> Json<Option<types::Profile>> {
    let filter = bson::doc! {"id": id};
    match app.profiles.find_one(Some(filter), None).await {
        Ok(profile) => Json(profile),
        Err(e) => {
            println!("{}", e);
            Json(None)
        }
    }
}

#[post("/update", format = "application/json", data = "<profile>")]
async fn upsert_profile(app: &State<App>, profile: Json<types::Profile>) -> Json<String> {
    // TODO: verify token here
    let profile = profile.into_inner();
    let result = app.profiles.update_one(, None).await;
    match result {
        Ok(_) => Json(profile.id),
        Err(e) => {
            println!("{}", e);
            Json(profile.id)
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let cfg = config::get();

    let mongo = connect_mongo(cfg).await.expect("failed to connect to mongo");
    let db = mongo.database(cfg.mongo_db);
    let profiles = db.collection(cfg.profiles_collection);

    rocket::build()
        .manage(App { mongo, profiles })
    .mount("/", routes![health, get_profile, upsert_profile])
}

async fn connect_mongo(cfg: &'static config::Config) -> Result<mongodb::Client, mongodb::error::Error> {
    let client_options = ClientOptions::parse(cfg.mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}