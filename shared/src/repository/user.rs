use crate::db::connection::DbConnection;
use crate::db::schema::users;
use crate::error::app_error::AppResult;
use crate::models::user::User;
use diesel::RunQueryDsl;

pub struct UserRepository;

impl UserRepository {
    pub fn get_all(conn: &mut DbConnection) -> AppResult<Vec<User>> {
        let res = users::table.load::<User>(conn)?;
        Ok(res)
    }
}
