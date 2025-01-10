use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRuleError(String),
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Not found: {0}")]
    NotFound(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(_) => {
                HttpResponse::BadRequest().json(ErrorResponse::from(self))
            }
            ApiError::BusinessRuleError(_) => {
                HttpResponse::UnprocessableEntity().json(ErrorResponse::from(self))
            }
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse::from(self))
            }
            ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(ErrorResponse::from(self))
            }
        }
    }
}
