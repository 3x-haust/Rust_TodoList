use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

use crate::db::DbPool;
use crate::models;
use crate::schema;

pub async fn create_todo(pool: web::Data<DbPool>, new_todo: web::Json<models::NewTodo>) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let todo = web::block(move || {
        diesel::insert_into(schema::todos::table)
            .values(&new_todo.into_inner())
            .execute(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(todo))
}

pub async fn list_todos(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let todos = web::block(move || {
        schema::todos::table
            .load::<models::Todo>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(todos))
}

pub async fn update_todo(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    updated_todo: web::Json<models::UpdateTodo>
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let todo = web::block(move || {
        diesel::update(schema::todos::table.find(id))
            .set(&updated_todo.into_inner())
            .execute(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if todo == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::Ok().json(json!({"status": "success"})))
    }
}

pub async fn delete_todo(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || {
        diesel::delete(schema::todos::table.find(id))
            .execute(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if result == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::Ok().json(json!({"status": "success"})))
    }
}