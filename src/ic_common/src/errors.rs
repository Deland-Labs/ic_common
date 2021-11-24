use candid::*;
use thiserror::Error;

#[cfg(test)]
mod tests;

pub trait IServiceError {
    // to ErrorInfo
    fn to_error_info(&self) -> ErrorInfo;
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Error)]
pub enum ServiceError {
    #[error("there is a unknown error raised")]
    Unknown,
    #[error("error from remote, {0:?}")]
    RemoteError(ErrorInfo),
    #[error("permission deny")]
    PermissionDenied,
    #[error("Length of {field:?} must be in range [{min:?}, {max:?})")]
    ValueShouldBeInRangeError {
        field: String,
        min: usize,
        max: usize,
    },
}

impl ServiceError {
    pub fn code(&self) -> u32 {
        match self {
            ServiceError::Unknown => 10001,
            ServiceError::RemoteError(_) => 10002,
            ServiceError::PermissionDenied => 10003,
            ServiceError::ValueShouldBeInRangeError { .. } => 10004,
        }
    }
}

impl IServiceError for ServiceError {
    fn to_error_info(&self) -> ErrorInfo {
        ErrorInfo {
            code: self.code(),
            message: self.to_string(),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize)]
pub struct ErrorInfo {
    pub code: u32,
    pub message: String,
}

pub type ServiceResult<T, E = ServiceError> = anyhow::Result<T, E>;

pub type ActorResult<T> = std::result::Result<T, ErrorInfo>;


impl From<ErrorInfo> for ServiceError {
    fn from(error: ErrorInfo) -> Self {
        ServiceError::RemoteError(error)
    }
}

pub fn to_actor_result<T, E>(result: ServiceResult<T, E>) -> ActorResult<T>
    where E: IServiceError {
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(error.to_error_info()),
    }
}
