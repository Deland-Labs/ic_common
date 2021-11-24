use thiserror::Error;

use ic_common::errors::{ErrorInfo, IServiceError, ServiceResult};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Error)]
pub(crate) enum UserServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
}

impl UserServiceError {
    pub fn code(&self) -> u32 {
        match self {
            UserServiceError::UserNotFound => 1,
            UserServiceError::UserAlreadyExists => 2,
        }
    }
}


impl IServiceError for UserServiceError {
    fn to_error_info(&self) -> ErrorInfo {
        ErrorInfo {
            code: self.code(),
            message: self.to_string(),
        }
    }
}

pub(crate) type UserServiceResult<T> = ServiceResult<T, UserServiceError>;