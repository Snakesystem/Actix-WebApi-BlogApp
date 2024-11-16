use actix_web::{ get, post, web::{ Data, Json, Path }, HttpResponse, Responder };
use serde::Deserialize;
use crate:: { messages:: { FetchUser, FetchUserArticles, CreateArticleUser }, AppState, DbActor };

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

use actix::Addr;

#[get("/users")]
pub async  fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET All user".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::NotFound().json("User not found"),
        error => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/user/{id}/articles")]
pub async  fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /user/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::NotFound().json(error.to_string()),
        error => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[post("/user/{id}/articles")]
pub async  fn create_user_article(state: Data<AppState>, path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("POST /user/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateArticleUser { title: body.title.to_string(), content: body.content.to_string(), created_by: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(error)) => HttpResponse::NotFound().json(error.to_string()), 
        error => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}