use async_trait::async_trait;
use jsonschema::JSONSchema;
use serde_json::Value;

use crate::domain::payment::PaymentRequest;
use crate::error::ValidationError;

#[async_trait]
pub trait PaymentValidator: Send + Sync {
    async fn validate(&self, request: &PaymentRequest) -> Result<(), ValidationError>;
    async fn validate_business_rules(&self, request: &PaymentRequest) -> Result<(), ValidationError>;
}

pub struct ISO20022PaymentValidator {
    schemas: HashMap<String, JSONSchema>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::payment::{PaymentRequest, PaymentType};
    use serde_json::json;

    struct MockISO20022PaymentValidator;

    #[async_trait]
    impl PaymentValidator for MockISO20022PaymentValidator {
        async fn validate(&self, _request: &PaymentRequest) -> Result<(), ValidationError> {
            Ok(())
        }

        async fn validate_business_rules(&self, _request: &PaymentRequest) -> Result<(), ValidationError> {
            Ok(())
        }
    }
}
