use crate::error::app_error::AppError::DatabaseError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use derive_more::derive::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Serialize;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Өгөгдлийн сангийн алдаа: {}", _0)]
    DatabaseError(String),

    #[display("Олдсонгүй: {}", _0)]
    NotFound(String),

    #[display("Баталгаажуулалтын алдаа: {}", _0)]
    ValidationError(String),

    #[display("Зөвшөөрөлгүй: {}", _0)]
    Unauthorized(String),

    #[display("Буруу хүсэлт: {}", _0)]
    BadRequest(String),

    #[display("Дотоод серверийн алдаа: {}", _0)]
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    status: u16,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();
        let error_response = ErrorResponse {
            error: match self {
                AppError::DatabaseError(_) => "DATABASE_ERROR",
                AppError::NotFound(_) => "NOT_FOUND",
                AppError::ValidationError(_) => "VALIDATION_ERROR",
                AppError::Unauthorized(_) => "UNAUTHORIZED",
                AppError::BadRequest(_) => "BAD_REQUEST",
                AppError::InternalError(_) => "INTERNAL_ERROR",
            }
            .to_string(),
            message: self.to_string(),
            status: status.as_u16(),
        };

        HttpResponse::build(status)
            .insert_header(ContentType::json())
            .json(error_response)
    }
}

// Diesel error conversion
impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => AppError::NotFound("Resource not found".to_string()),
            DieselError::DatabaseError(kind, _info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    DatabaseError("Энэ утга аль хэдийн бүртгэгдсэн байна".to_string())
                }
                _ => DatabaseError("Internal server error".to_string()),
            },
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// r2d2 pool error conversion
impl From<diesel::r2d2::PoolError> for AppError {
    fn from(err: diesel::r2d2::PoolError) -> Self {
        AppError::DatabaseError(format!("Connection pool error: {}", err))
    }
}

// Standard error conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// Result type alias
pub type AppResult<T> = Result<T, AppError>;
