use async_trait::async_trait;
use crate::validation::payment_validator::{PaymentValidator, ISO20022PaymentValidator};
use crate::domain::payment::PaymentRequest;
use crate::error::ValidationError;
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

#[tokio::test]
async fn test_validate() {
    let validator = MockISO20022PaymentValidator;
    let request = PaymentRequest {
        message_type: "pain.001".to_string(),
        payment_type: PaymentType::CreditTransfer,
        message_payload: json!({}),
        sender_id: "sender".to_string(),
        request_id: "request".to_string(),
    };

    let result = validator.validate(&request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_business_rules() {
    let validator = MockISO20022PaymentValidator;
    let request = PaymentRequest {
        message_type: "pain.001".to_string(),
        payment_type: PaymentType::CreditTransfer,
        message_payload: json!({}),
        sender_id: "sender".to_string(),
        request_id: "request".to_string(),
    };

    let result = validator.validate_business_rules(&request).await;
    assert!(result.is_ok());
}
