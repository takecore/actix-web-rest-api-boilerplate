use std::fmt;

use actix_web::{error::BlockingError, error::ResponseError, http::StatusCode, HttpResponse};
use diesel::result::Error as DieselError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    status: u16,
    title: String,
    #[serde(skip_serializing)]
    kind: AppErrorKind,
}

#[derive(Debug)]
pub enum AppErrorKind {
    NotFoundError,
    DatabaseError,
    InternalServerError,
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.to_string())
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::NotFound => AppError {
                status: StatusCode::NOT_FOUND.as_u16(),
                title: error.to_string(),
                kind: AppErrorKind::NotFoundError,
            },
            _ => {
                error!("Database error: {}", error);
                AppError {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    title: error.to_string(),
                    kind: AppErrorKind::DatabaseError,
                }
            }
        }
    }
}

impl<E> From<BlockingError<E>> for AppError
where
    E: std::fmt::Debug,
    E: Into<AppError>,
{
    fn from(error: BlockingError<E>) -> AppError {
        error!("actix threadpool pool error: {}", error);
        match error {
            BlockingError::Error(e) => e.into(),
            BlockingError::Canceled => AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                title: error.to_string(),
                kind: AppErrorKind::InternalServerError,
            },
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            AppErrorKind::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorKind::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorKind::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        // TODO if 500 error then logging
        let stauts = self.status_code();
        HttpResponse::build(stauts).json(self)
    }
}
