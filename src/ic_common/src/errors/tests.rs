use rstest::*;
use thiserror::Error;

use super::*;

mod default_error{
    use super::*;

    #[rstest]
    fn test_to_actor_result() {
        let result: ServiceResult<()> = Err(ServiceError::Unknown);
        let result = to_actor_result(result).unwrap_err();
        assert_eq!(result.code, 10001);
        assert_eq!(result.message, "there is a unknown error raised");
    }
}

mod custom_error {
    use crate::errors::{ErrorInfo, IServiceError, ServiceResult, to_actor_result};

    use super::*;

    #[derive(Error, Debug)]
    enum CustomError {
        #[error("Custom error1")]
        CustomError1,
        #[error("Custom error2")]
        CustomError2,
    }

    impl IServiceError for CustomError {
        fn to_error_info(&self) -> ErrorInfo {
            ErrorInfo {
                code: self.code(),
                message: self.to_string(),
            }
        }
    }

    impl CustomError {
        pub fn code(&self) -> u32 {
            match self {
                CustomError::CustomError1 => 1,
                CustomError::CustomError2 => 2,
            }
        }
    }

    #[rstest]
    fn test_custom_error_to_result() {
        let result: ServiceResult<(), CustomError> = Err(CustomError::CustomError1);
        let result = to_actor_result(result).unwrap_err();
        assert_eq!(result.code, 1);
        assert_eq!(result.message, "Custom error1");
    }
}