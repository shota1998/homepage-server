use crate::diesel;

use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

/// Get json from user. Then store it to DB.
/// 
/// #Arguments
/// * new_user (Json<NewUsrSchema>): json body.
/// 
/// # Returns
/// * (HttpResponse): response body to be passed to the viwer.
pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
  let connection = establish_connection();
  // Prepare data which will be stored to DB.
  let name:     String = new_user.name.clone();
  let email:    String = new_user.email.clone();
  let password: String = new_user.password.clone();
  let new_user = NewUser::new(name, email, password);
  // Store data to DB.
  let insert_result = diesel::insert_into(users::table)
                              .values(&new_user)
                              .execute(&connection);
  // Storing was succeded or not.
  match  insert_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}