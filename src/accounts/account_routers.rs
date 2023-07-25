use crate::accounts::{Account, Accounts};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/accounts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let accounts = web::block(|| Accounts::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(accounts))
}

#[get("/accounts/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[post("/accounts")]
async fn create(account: web::Json<Account>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::create(account.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[put("/accounts/{id}")]
async fn update(
    id: web::Path<i32>,
    account: web::Json<Account>,
) -> Result<HttpResponse, CustomError> {
    let account = Accounts::update(id.into_inner(), account.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_account = Accounts::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_account })))
}

pub fn init_account_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}

