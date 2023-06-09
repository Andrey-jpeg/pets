mod database;
mod entities;
mod pet;

use database::init_db;
use dotenv::dotenv;
use sea_orm::{DatabaseConnection};

#[derive(Clone)]
pub struct State {
    db: DatabaseConnection,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db = match init_db(connection_url).await {
          Ok(db) => db,
          Err(err) => panic!("{}", err)
    };

    let state = State { db };
    femme::start();

    let mut app = tide::with_state(state);
    app.at("/pet/:id").get(pet::cat_controller::read);
    app.at("/pet/:id").put(pet::cat_controller::update);
    app.at("/pet/:id").delete(pet::cat_controller::delete);
    app.at("/pet").post(pet::cat_controller::create);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}