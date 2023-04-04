use sea_orm::{EntityTrait, ModelTrait};
use tide::{Request, Response};


use crate::entities::prelude::Pets;
use crate::State;

// @TODO Create
// pub async fn _create(_req: Request<State>) -> tide::Result<> {
//     let res = Response::new(400);

//     Ok(res)
// }

pub async fn read(req: Request<State>) -> tide::Result<> {
    let mut res = Response::new(400);
    let id = req.param("id")?;
    let my_int= id.parse::<i32>();

    if my_int.is_err(){
        res.set_body( "Found '".to_string() + id + "', expected a number");
        return Ok(res)
    }

    let pet = Pets::find_by_id(my_int.unwrap()).one(&req.state().db).await?;

    if pet.is_none() {
        res.set_status(tide::StatusCode::NotFound);
        return Ok(res)
    }
    res.set_body(pet.unwrap().pet_name);
    res.set_status(tide::StatusCode::Ok);

    Ok(res)
}
// @TODO: update
// pub async fn _update(_req: Request<State>) -> tide::Result<> {
//     let res = Response::new(400);

//     Ok(res)
// }

pub async fn delete(req: Request<State>) -> tide::Result<> {
    let mut res = Response::new(400);
    let id = req.param("id")?;
    let my_int= id.parse::<i32>();

    if my_int.is_err(){
        res.set_body( "Found '".to_string() + id + "', expected a number");
        return Ok(res)
    }

    let pet = Pets::find_by_id(my_int.unwrap()).one(&req.state().db).await?;
    let pet = pet.unwrap();
    let result = pet.delete(&req.state().db).await?;
    res.set_body("Rows affected: ".to_string() + &result.rows_affected.to_string());
    Ok(res)
}