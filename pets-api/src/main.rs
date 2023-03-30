mod database;
mod entities;

use database::init_db;
use dotenv::dotenv;
use sea_orm::{DatabaseConnection, EntityTrait};
use tide::{Request, Response};

use entities::{prelude::*};

#[derive(Clone)]
struct State {
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

    let mut app = tide::with_state(state);

    app.at("/pet/:id").get(get_pet);
    

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn get_pet(req: Request<State>) -> tide::Result<> {
    let mut res = Response::new(tide::StatusCode::Ok);

    let id = req.param("id")?;
    let my_int :i32 = id.parse::<i32>().unwrap();

    let pet = Pets::find_by_id(my_int).one(&req.state().db).await?;
    res.set_body(pet.unwrap().pet_name);
    
    Ok(res)
}