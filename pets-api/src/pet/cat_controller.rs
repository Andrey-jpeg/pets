use sea_orm::{EntityTrait, ModelTrait, Set, ActiveModelTrait};
use tide::{Request, Response };


use crate::entities::cat;
use crate::entities::prelude::Cat;
use crate::State;

#[derive(serde::Deserialize)]
struct CatReq{
    pet_name: String,
    color: Option<String>,
    status: Option<String>
}

pub async fn create(mut req: Request<State>) -> tide::Result<> {
    let req_cat: CatReq = req.body_json().await?;

    let new_cat = cat::ActiveModel {
        pet_name: Set(req_cat.pet_name),
        color: Set(req_cat.color),
        status: Set(req_cat.status),
        ..Default::default()
    };

    new_cat.save(&req.state().db).await?;   

    Ok(Response::new(201))
}

pub async fn update(mut req: Request<State>) -> tide::Result<> {
    let id = req.param("id")?;
    let id = id.parse::<i32>();
    let req_cat: CatReq = req.body_json().await?;

    let cat = Cat::find_by_id(id.unwrap()).one(&req.state().db).await?;

    let mut cat: cat::ActiveModel = cat.unwrap().into();
    cat.pet_name = Set(req_cat.pet_name);
    if req_cat.color.is_some() {
        cat.color = Set(req_cat.color);
    }
    if req_cat.status.is_some() {
        cat.status = Set(req_cat.status);
    }

    cat.update(&req.state().db).await?;

    Ok(Response::new(200))
}

// @TODO Fix ugly handling of JSON serialization.
pub async fn read(req: Request<State>) -> tide::Result<> {
    let mut res = tide::Response::new(tide::StatusCode::Accepted);
    let id = req.param("id")?;
    let my_int= id.parse::<i32>();

    if my_int.is_err(){
        res.set_status(tide::StatusCode::InternalServerError);
        res.set_body( "Found '".to_string() + id + "', expected a number");

        return Ok(res);
    }

    let pet = Cat::find_by_id(my_int.unwrap()).one(&req.state().db).await?;

    if pet.is_none() {
        res.set_body(tide::Body::from_string("no pet with id: ".to_owned() +  id));
        res.set_status(tide::StatusCode::NotFound);
        return Ok(res);
    }

    let json = serde_json::to_value(&pet.unwrap());
    
    res.set_body(tide::Body::from_json(&json.unwrap()).unwrap());
    res.set_status(tide::StatusCode::Ok);
    
    Ok(res)
}

pub async fn delete(req: Request<State>) -> tide::Result<> {
    let mut res = Response::new(400);
    let id = req.param("id")?;
    let my_int= id.parse::<i32>();

    if my_int.is_err(){
        res.set_body( "Found '".to_string() + id + "', expected a number");
        return Ok(res)
    }

    let pet = Cat::find_by_id(my_int.unwrap()).one(&req.state().db).await?;
    let pet = pet.unwrap();
    let result = pet.delete(&req.state().db).await?;
    res.set_body("Rows affected: ".to_string() + &result.rows_affected.to_string());
    Ok(res)
}