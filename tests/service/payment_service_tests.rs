use async_trait::async_trait;
use crate::service::payment_service::{PaymentService, PaymentServiceImpl};
use crate::domain::payment::{PaymentRequest, PaymentResponse, PaymentStatus};
use crate::error::ServiceError;
use crate::validation::PaymentValidator;
use crate::infrastructure::messaging::MessagePublisher;
use uuid::Uuid;
use serde_json::json;

struct MockPaymentValidator;

#[async_trait]
impl PaymentValidator for MockPaymentValidator {
    async fn validate(&self, _request: &PaymentRequest) -> Result<(), ValidationError> {
        Ok(())
    }

    async fn validate_business_rules(&self, _request: &PaymentRequest) -> Result<(), ValidationError> {
        Ok(())
    }
}

struct MockMessagePublisher;

#[async_trait]
impl MessagePublisher for MockMessagePublisher {
    async fn publish_message(&self, _routing_key: &str, _message: serde_json::Value) -> Result<(), MessagingError> {
        Ok(())
    }
}

struct MockPaymentRepository;

#[async_trait]
impl PaymentRepository for MockPaymentRepository {
    async fn save_payment(&self, _payment: Payment) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn get_payment(&self, _id: &Uuid) -> Result<Option<Payment>, RepositoryError> {
        Ok(None)
    }

    async fn update_status(&self, _id: &Uuid, _status: PaymentStatus) -> Result<(), RepositoryError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_process_payment() {
    let validator = Box::new(MockPaymentValidator);
    let message_publisher = Box::new(MockMessagePublisher);
    let repository = Box::new(MockPaymentRepository);
    let service = PaymentServiceImpl {
        validator,
        message_publisher,
        repository,
    };

    let request = PaymentRequest {
        message_type: "pain.001".to_string(),
        payment_type: PaymentType::CreditTransfer,
        message_payload: json!({}),
        sender_id: "sender".to_string(),
        request_id: "request".to_string(),
    };

    let result = service.process_payment(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_status() {
    let validator = Box::new(MockPaymentValidator);
    let message_publisher = Box::new(MockMessagePublisher);
    let repository = Box::new(MockPaymentRepository);
    let service = PaymentServiceImpl {
        validator,
        message_publisher,
        repository,
    };

    let payment_id = Uuid::new_v4();
    let result = service.get_status(&payment_id).await;
    assert!(result.is_ok());
}
