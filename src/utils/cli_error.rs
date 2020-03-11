use actix_web::{
    dev::HttpResponseBuilder,
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};
use bson::oid::Error as BsonObjectIdError;
use bson::{DecoderError, EncoderError};
use failure::Fail;
use mongodb::error::Error as MongoDbError;
use std::fmt;

#[derive(Fail, Debug)]
pub enum CliError {
    BsonObjectIdError(BsonObjectIdError),
    EncoderError(EncoderError),
    DecoderError(DecoderError),
    MongoDbError(MongoDbError),
    InvalidDataError(String),
    NotFoundError,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::BsonObjectIdError(ref err) => write!(f, "{}", err),
            CliError::EncoderError(ref err) => write!(f, "{}", err),
            CliError::DecoderError(ref err) => write!(f, "{}", err),
            CliError::MongoDbError(ref err) => write!(f, "{}", err),
            CliError::InvalidDataError(ref name) => write!(f, "Invalid Data {}", name),
            CliError::NotFoundError => write!(f, "Not Found"),
        }
    }
}

impl ResponseError for CliError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CliError::BsonObjectIdError(_) => StatusCode::BAD_REQUEST,
            CliError::EncoderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CliError::DecoderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CliError::MongoDbError(_) => StatusCode::BAD_GATEWAY,
            CliError::InvalidDataError(_) => StatusCode::BAD_REQUEST,
            CliError::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
}

impl From<EncoderError> for CliError {
    fn from(error: EncoderError) -> CliError {
        CliError::EncoderError(error)
    }
}

impl From<DecoderError> for CliError {
    fn from(error: DecoderError) -> CliError {
        CliError::DecoderError(error)
    }
}

impl From<MongoDbError> for CliError {
    fn from(error: MongoDbError) -> CliError {
        CliError::MongoDbError(error)
    }
}

impl From<BsonObjectIdError> for CliError {
    fn from(error: BsonObjectIdError) -> CliError {
        CliError::BsonObjectIdError(error)
    }
}
