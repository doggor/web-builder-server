use crate::utils::CliError;
use std::result;

pub type Result<T> = result::Result<T, CliError>;
