use actix_web::http::StatusCode;
use actix_web::{HttpResponse, error};
use derive_more::Display;
use serde_json::json;

#[derive(Display, Debug)]
pub enum UserError {
    #[display("Invalid input parameter")]
    ValidationError,
    #[display("Internal server error")]
    DBPoolGetError,
    #[display("Not found")]
    NotFoundError,
    #[display("Internal server error")]
    UnexpectedError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({ "msg": self.to_string() }))
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError => StatusCode::BAD_REQUEST,
            UserError::DBPoolGetError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NotFoundError => StatusCode::NOT_FOUND,
            UserError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
