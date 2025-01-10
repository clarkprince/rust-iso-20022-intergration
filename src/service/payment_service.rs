use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::payment::{PaymentRequest, PaymentResponse, PaymentStatus};
use crate::error::ServiceError;
use crate::validation::PaymentValidator;
use crate::infrastructure::messaging::MessagePublisher;

#[async_trait]
pub trait PaymentService: Send + Sync {
    async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResponse, ServiceError>;
    async fn get_status(&self, payment_id: &Uuid) -> Result<PaymentStatus, ServiceError>;
}

pub struct PaymentServiceImpl {
    validator: Box<dyn PaymentValidator>,
    message_publisher: Box<dyn MessagePublisher>,
    repository: Box<dyn PaymentRepository>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::payment::{PaymentRequest, PaymentType};
    use crate::error::ServiceError;
    use async_trait::async_trait;
    use serde_json::json;
    use uuid::Uuid;

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

    // Remove the test functions
}
