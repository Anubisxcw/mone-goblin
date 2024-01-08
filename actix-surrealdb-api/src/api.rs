use actix_web::{
    delete,
    get,
    patch,
    post,
    web,
    web::{Json, Path},
    // HttpResponse,
};
use types::*;

use crate::db::*;
use crate::prelude::*;

#[post("/inv")]
pub async fn create(inv: web::Json<Investment>) -> Result<Json<Investment>> {
    let mut inv = inv.into_inner();
    let todo = add_inv(&mut inv).await?;
    Ok(Json(todo))

    // match todo_id {
    //     Ok(id) => HttpResponse::Ok().json(id),
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

#[get("/inv/{id}")]
pub async fn get(id: Path<String>) -> Result<Json<Investment>> {
    let task = get_inv(id.into_inner()).await?;

    Ok(Json(task))
}

#[patch("/inv")]
pub async fn update(inv: web::Json<Investment2>) -> Result<Json<Investment>> {
    println!("meaw2");
    let mut inv = inv.into_inner();
    let updated = update_inv(&mut inv).await?;

    Ok(Json(updated))
}

#[delete("/inv/{id}")]
pub async fn delete(id: Path<String>) -> Result<Json<AffectedRows>> {
    let deleted = delete_inv(id.into_inner()).await?;

    Ok(Json(deleted))
}

#[get("/invs")]
pub async fn list() -> Result<Json<Vec<Investment>>> {
    let todos = get_all_invs().await?;
    println!("meaw");
    Ok(Json(todos))
}
