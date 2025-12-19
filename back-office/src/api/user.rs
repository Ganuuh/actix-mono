use actix_web::{HttpResponse, web};
use shared::{
    common::presenter::presenter, db::connection::DbPool, error::app_error::AppResult,
    repository::user::UserRepository,
};

pub async fn get_all(state: web::Data<DbPool>) -> AppResult<HttpResponse> {
    let pool = &mut state.get()?;
    let users = UserRepository::get_all(pool)?;
    Ok(presenter(
        users,
        Some("Users retrieved successfully".to_string()),
    ))
}

pub async fn create(
    state: web::Data<DbPool>,
    new_user: web::Json<shared::models::user::NewUser>,
) -> AppResult<HttpResponse> {
    let pool = &mut state.get()?;
    let user = UserRepository::create(new_user.into_inner(), pool)?;
    Ok(presenter(
        user,
        Some("User created successfully".to_string()),
    ))
}
