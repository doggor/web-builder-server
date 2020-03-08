use actix_web::HttpResponse;
use serde::Serialize;
use crate::utils::CliError;

pub fn ok<T: Serialize>(value: T) -> Result<HttpResponse, CliError> {
    Ok(HttpResponse::Ok().json(value))
}
