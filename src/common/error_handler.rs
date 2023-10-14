use actix_web::{
    http::StatusCode
};
use bcrypt::BcryptError;
use wither::WitherError;
use wither::mongodb::error::Error as MongoError;
use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not Found")]
pub struct NotFound {}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credential")]
    WrongCredentials,

    #[error("failed to create authentication token")]
    TokenCreation,

    #[error("Invalid token credential")]
    InvalidToken,

    #[error("User is Locked")]
    Locked,
}


#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Wither(#[from] WitherError),

    #[error("{0}")]
    Mongo(#[from] MongoError),

    #[error("{0}")]
    SerializeMongoResponse(#[from] bson::de::Error),

    #[error("{0}")]
    ParseObjectID(String),

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),
}

impl Error {
    fn get_code(&self) -> (StatusCode, u16) {
        match *self {
            // 4xx errors
            Error::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            Error::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40004),
            Error::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40005),
            Error::Authenticate(AuthenticateError::Locked) => (StatusCode::UNAUTHORIZED, 40006),
           // 5xx errors
            Error::Authenticate(AuthenticateError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, 5001),
            Error::Wither(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5002),
            Error::Mongo(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5003),
            Error::SerializeMongoResponse(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5004),
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
        }
    }
}

