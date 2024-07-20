//! This module provides implementations for conversion and display traits related to FCM operations.

use crate::error::FcmError;
use crate::models::{FcmErrorResponse, FcmSendResult, FcmSuccessResponse};
use std::fmt::Display;

impl Into<Result<FcmSuccessResponse, FcmError>> for FcmSendResult {
    fn into(self) -> Result<FcmSuccessResponse, FcmError> {
        match self {
            FcmSendResult::Success(success) => Ok(success),
            FcmSendResult::Error(error) => Err(FcmError::ResponseError(error)),
        }
    }
}

impl Display for FcmErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(&self))
    }
}
