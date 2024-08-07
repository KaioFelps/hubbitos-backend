use log::error;
use crate::{errors::{error::DomainErrorTrait, internal_error::InternalError}, LOG_SEP, R_EOL};
use std::error::Error;

pub fn generate_service_internal_error(message: &str, error: &Box<dyn Error>) -> Box<dyn DomainErrorTrait>{
    error!(
        "{R_EOL}{LOG_SEP}{R_EOL}{}: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
        message, error
    );

    return Box::new(InternalError::new());
}