use actix_web::{web, HttpResponse, Scope};
use tracing::{info, error};
use uuid::Uuid;

use crate::domain::payment::{
    PaymentRequest, PaymentResponse, CreditTransferRequest, DirectDebitRequest,
    InstantPaymentRequest, BulkPaymentRequest, MandateRequest
};
use crate::error::ApiError;
use crate::service::{
    payment::PaymentService,
    credit_transfer::CreditTransferService,
    direct_debit::DirectDebitService,
    instant_payment::InstantPaymentService,
    bulk_payment::BulkPaymentService,
    mandate::MandateService
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/payments")
                    .service(submit_payment)
                    .service(get_payment_status)
                    .service(search_payments)
            )
            .service(
                web::scope("/credit-transfers")
                    .service(submit_credit_transfer)
                    .service(get_credit_transfer_status)
                    .service(cancel_credit_transfer)
            )
            .service(
                web::scope("/direct-debits")
                    .service(submit_direct_debit)
                    .service(get_direct_debit_status)
                    .service(cancel_direct_debit)
                    .service(manage_mandate)
            )
            .service(
                web::scope("/instant-payments")
                    .service(submit_instant_payment)
                    .service(get_instant_payment_status)
                    .service(verify_instant_payment)
            )
            .service(
                web::scope("/bulk-payments")
                    .service(submit_bulk_payment)
                    .service(get_bulk_status)
                    .service(cancel_bulk_payment)
            )
            .service(
                web::scope("/mandates")
                    .service(create_mandate)
                    .service(update_mandate)
                    .service(cancel_mandate)
                    .service(get_mandate_status)
            )
    );
}

#[post("")]
async fn submit_payment(
    payment_request: web::Json<PaymentRequest>,
    payment_service: web::Data<PaymentService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received payment submission request");
    
    let response = payment_service
        .process_payment(payment_request.into_inner())
        .await
        .map_err(|e| {
            error!("Payment processing failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/{payment_id}/status")]
async fn get_payment_status(
    payment_id: web::Path<Uuid>,
    payment_service: web::Data<PaymentService>,
) -> Result<HttpResponse, ApiError> {
    let status = payment_service
        .get_status(&payment_id)
        .await
        .map_err(|e| {
            error!("Failed to retrieve payment status: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(status))
}

// Credit Transfer APIs
#[post("/credit-transfers")]
async fn submit_credit_transfer(
    request: web::Json<CreditTransferRequest>,
    service: web::Data<CreditTransferService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received credit transfer request");
    
    let response = service
        .process_credit_transfer(request.into_inner())
        .await
        .map_err(|e| {
            error!("Credit transfer processing failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/credit-transfers/{transfer_id}/status")]
async fn get_credit_transfer_status(
    transfer_id: web::Path<Uuid>,
    service: web::Data<CreditTransferService>,
) -> Result<HttpResponse, ApiError> {
    let status = service
        .get_credit_transfer_status(&transfer_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(status))
}

#[delete("/credit-transfers/{transfer_id}")]
async fn cancel_credit_transfer(
    transfer_id: web::Path<Uuid>,
    service: web::Data<CreditTransferService>,
) -> Result<HttpResponse, ApiError> {
    service
        .cancel_credit_transfer(&transfer_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().finish())
}

// Direct Debit APIs
#[post("/direct-debits")]
async fn submit_direct_debit(
    request: web::Json<DirectDebitRequest>,
    service: web::Data<DirectDebitService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received direct debit request");
    
    let response = service
        .process_direct_debit(request.into_inner())
        .await
        .map_err(|e| {
            error!("Direct debit processing failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/direct-debits/{debit_id}/status")]
async fn get_direct_debit_status(
    debit_id: web::Path<Uuid>,
    service: web::Data<DirectDebitService>,
) -> Result<HttpResponse, ApiError> {
    let status = service
        .get_direct_debit_status(&debit_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(status))
}

#[delete("/direct-debits/{debit_id}")]
async fn cancel_direct_debit(
    debit_id: web::Path<Uuid>,
    service: web::Data<DirectDebitService>,
) -> Result<HttpResponse, ApiError> {
    service
        .cancel_direct_debit(&debit_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().finish())
}

// Instant Payment APIs
#[post("/instant-payments")]
async fn submit_instant_payment(
    request: web::Json<InstantPaymentRequest>,
    service: web::Data<InstantPaymentService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received instant payment request");
    
    let response = service
        .process_instant_payment(request.into_inner())
        .await
        .map_err(|e| {
            error!("Instant payment processing failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/instant-payments/{payment_id}/status")]
async fn get_instant_payment_status(
    payment_id: web::Path<Uuid>,
    service: web::Data<InstantPaymentService>,
) -> Result<HttpResponse, ApiError> {
    let status = service
        .get_instant_payment_status(&payment_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(status))
}

#[post("/instant-payments/{payment_id}/verify")]
async fn verify_instant_payment(
    payment_id: web::Path<Uuid>,
    service: web::Data<InstantPaymentService>,
) -> Result<HttpResponse, ApiError> {
    let verification = service
        .verify_instant_payment(&payment_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(verification))
}

// Bulk Payment APIs
#[post("/bulk-payments")]
async fn submit_bulk_payment(
    request: web::Json<BulkPaymentRequest>,
    service: web::Data<BulkPaymentService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received bulk payment request");
    
    let response = service
        .process_bulk_payment(request.into_inner())
        .await
        .map_err(|e| {
            error!("Bulk payment processing failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/bulk-payments/{bulk_id}/status")]
async fn get_bulk_status(
    bulk_id: web::Path<Uuid>,
    service: web::Data<BulkPaymentService>,
) -> Result<HttpResponse, ApiError> {
    let status = service
        .get_bulk_payment_status(&bulk_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(status))
}

#[delete("/bulk-payments/{bulk_id}")]
async fn cancel_bulk_payment(
    bulk_id: web::Path<Uuid>,
    service: web::Data<BulkPaymentService>,
) -> Result<HttpResponse, ApiError> {
    service
        .cancel_bulk_payment(&bulk_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().finish())
}

// Mandate Management APIs
#[post("/mandates")]
async fn create_mandate(
    request: web::Json<MandateRequest>,
    service: web::Data<MandateService>,
) -> Result<HttpResponse, ApiError> {
    info!("Received mandate creation request");
    
    let response = service
        .create_mandate(request.into_inner())
        .await
        .map_err(|e| {
            error!("Mandate creation failed: {:?}", e);
            e.into()
        })?;

    Ok(HttpResponse::Created().json(response))
}

#[put("/mandates/{mandate_id}")]
async fn update_mandate(
    mandate_id: web::Path<Uuid>,
    request: web::Json<MandateRequest>,
    service: web::Data<MandateService>,
) -> Result<HttpResponse, ApiError> {
    let response = service
        .update_mandate(&mandate_id, request.into_inner())
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/mandates/{mandate_id}")]
async fn cancel_mandate(
    mandate_id: web::Path<Uuid>,
    service: web::Data<MandateService>,
) -> Result<HttpResponse, ApiError> {
    service
        .cancel_mandate(&mandate_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/mandates/{mandate_id}/status")]
async fn get_mandate_status(
    mandate_id: web::Path<Uuid>,
    service: web::Data<MandateService>,
) -> Result<HttpResponse, ApiError> {
    let status = service
        .get_mandate_status(&mandate_id)
        .await
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(status))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::service::payment::MockPaymentService;
    use crate::domain::payment::{PaymentRequest, PaymentType};
    use serde_json::json;

    // Remove the test functions
}
