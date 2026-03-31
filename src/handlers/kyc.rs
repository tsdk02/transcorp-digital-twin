use actix_web::{web, HttpResponse};

use crate::models::customer::*;
use crate::models::envelope::ApiResponse;
use crate::models::error;
use crate::services::id_gen;
use crate::state::AppState;

pub async fn generate_otp(
    body: web::Json<GenerateOtpRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let mobile = match &body.mobile_number {
        Some(m) if !m.is_empty() => m.clone(),
        _ => return error::missing_field("mobileNumber").to_response(),
    };

    let otp = id_gen::generate_otp();
    let now = chrono::Utc::now().timestamp();

    log::info!("OTP generated for entity_id={}: {}", entity_id, otp);

    let record = OtpRecord {
        otp,
        entity_id: entity_id.clone(),
        mobile: mobile.clone(),
        expires_at: now + 300, // 5 minutes
    };

    {
        let mut otps = state.otps.write().unwrap();
        otps.insert(entity_id.clone(), record);
    }

    HttpResponse::Ok().json(ApiResponse::success(GenerateOtpResponse {
        success: true,
        entity_id,
        flow_ref_no: None,
    }))
}

pub async fn register_customer(
    body: web::Json<RegisterCustomerRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let otp = match &body.otp {
        Some(o) if !o.is_empty() => o.clone(),
        _ => return error::missing_field("otp").to_response(),
    };

    let first_name = match &body.first_name {
        Some(n) if !n.is_empty() => n.clone(),
        _ => return error::missing_field("firstName").to_response(),
    };

    let last_name = match &body.last_name {
        Some(n) if !n.is_empty() => n.clone(),
        _ => return error::missing_field("lastName").to_response(),
    };

    // Check if customer already exists
    {
        let customers = state.customers.read().unwrap();
        if customers.contains_key(&entity_id) {
            return error::customer_already_registered().to_response();
        }
    }

    // Validate OTP
    let now = chrono::Utc::now().timestamp();
    {
        let otps = state.otps.read().unwrap();
        match otps.get(&entity_id) {
            Some(record) => {
                if record.expires_at < now {
                    return error::otp_expired().to_response();
                }
                if record.otp != otp {
                    return error::invalid_otp().to_response();
                }
            }
            None => return error::invalid_otp().to_response(),
        }
    }

    // Determine kit_no
    let kit_no = body
        .kit_info
        .as_ref()
        .and_then(|kits| kits.first())
        .and_then(|k| k.kit_no.clone())
        .unwrap_or_else(id_gen::generate_kit_no);

    let token = id_gen::generate_token();

    // Extract mobile from communication_info
    let mobile = body
        .communication_info
        .as_ref()
        .and_then(|ci| ci.first())
        .and_then(|c| c.contact_no.clone())
        .unwrap_or_default();

    let email = body
        .communication_info
        .as_ref()
        .and_then(|ci| ci.first())
        .and_then(|c| c.email_id.clone());

    let customer = Customer {
        entity_id: entity_id.clone(),
        first_name: first_name.clone(),
        middle_name: body.middle_name.clone(),
        last_name: last_name.clone(),
        gender: body.gender.clone(),
        mobile,
        email,
        token: token.clone(),
        kyc_status: body
            .channel_name
            .clone()
            .unwrap_or_else(|| "MIN_KYC".to_string()),
        created_at: now,
    };

    // Store customer
    {
        let mut customers = state.customers.write().unwrap();
        customers.insert(entity_id.clone(), customer);
    }

    // Initialize balance
    {
        let mut balances = state.balances.write().unwrap();
        balances.insert(entity_id.clone(), 0);
    }

    // Create card from kit_info if provided
    if let Some(kit_infos) = &body.kit_info {
        for kit in kit_infos {
            let card = crate::models::card::Card {
                kit_no: kit.kit_no.clone().unwrap_or_else(id_gen::generate_kit_no),
                entity_id: entity_id.clone(),
                card_number: id_gen::generate_card_number(),
                card_type: kit
                    .card_type
                    .clone()
                    .unwrap_or_else(|| "VIRTUAL".to_string()),
                card_category: kit
                    .card_category
                    .clone()
                    .unwrap_or_else(|| "PREPAID".to_string()),
                status: match kit.card_reg_status.as_deref() {
                    Some("ACTIVE") => crate::models::card::CardStatus::Active,
                    Some("LOCKED") => crate::models::card::CardStatus::Locked,
                    Some("UNACTIVATED") => crate::models::card::CardStatus::Unactivated,
                    _ => crate::models::card::CardStatus::Active,
                },
                network_type: "VISA".to_string(),
                expiry_date: id_gen::generate_expiry(),
                physical_card_requested: false,
                created_at: now,
            };

            let mut cards = state.cards.write().unwrap();
            cards.insert(card.kit_no.clone(), card);
        }
    }

    // Clear OTP
    {
        let mut otps = state.otps.write().unwrap();
        otps.remove(&entity_id);
    }

    log::info!("Customer registered: entity_id={}", entity_id);

    HttpResponse::Ok().json(ApiResponse::success(RegisterCustomerResponse {
        entity_id,
        kit_no,
        token,
        valid: false,
    }))
}
