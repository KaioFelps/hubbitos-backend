use std::fmt;

use http::StatusCode;

#[derive(Debug, Clone)]
pub struct UnauthorizedError {
    code: u16,
    message: String,
}

impl UnauthorizedError {
    pub fn new() -> Self {
        UnauthorizedError {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: format!("Unauthorized.")
        }
    }

    pub fn code(&self) -> &u16 {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UnauthorizedError {}