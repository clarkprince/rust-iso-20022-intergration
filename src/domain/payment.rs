use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub message_type: String,
    pub payment_type: PaymentType,
    pub message_payload: serde_json::Value,
    pub sender_id: String,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentType {
    CreditTransfer,
    DirectDebit,
    RequestForPayment,
    PaymentReturn,
    RealTimePayment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditTransferRequest {
    pub amount: f64,
    pub currency: String,
    pub sender_account: String,
    pub receiver_account: String,
    pub transfer_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectDebitRequest {
    pub amount: f64,
    pub currency: String,
    pub sender_account: String,
    pub receiver_account: String,
    pub debit_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstantPaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub sender_account: String,
    pub receiver_account: String,
    pub payment_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkPaymentRequest {
    pub payments: Vec<PaymentRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MandateRequest {
    pub mandate_id: Uuid,
    pub debtor_account: String,
    pub creditor_account: String,
    pub mandate_date: DateTime<Utc>,
}
