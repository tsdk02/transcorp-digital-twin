use actix_web::{web, HttpResponse};

use crate::models::corporate::*;
use crate::models::envelope::ApiResponse;
use crate::models::error;
use crate::state::AppState;

pub async fn register_corporate(
    body: web::Json<RegisterCorporateRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let entity_type = match &body.entity_type {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return error::missing_field("entityType").to_response(),
    };

    let business_type = match &body.business_type {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return error::missing_field("businessType").to_response(),
    };

    let first_name = match &body.first_name {
        Some(n) if !n.is_empty() => n.clone(),
        _ => return error::missing_field("firstName").to_response(),
    };

    let kit_no = match &body.kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("kitNo").to_response(),
    };

    // Check if already exists
    {
        let corporates = state.corporates.read().unwrap();
        if corporates.contains_key(&entity_id) {
            return error::ErrorDetail::new("Y502", "Business already Exists").to_response();
        }
    }

    let now = chrono::Utc::now().timestamp();

    let corporate = Corporate {
        entity_id: entity_id.clone(),
        entity_type,
        business_type,
        first_name,
        last_name: body.last_name.clone(),
        kit_no: kit_no.clone(),
        created_at: now,
    };

    {
        let mut corporates = state.corporates.write().unwrap();
        corporates.insert(entity_id.clone(), corporate);
    }

    // Initialize balance
    {
        let mut balances = state.balances.write().unwrap();
        balances.insert(entity_id.clone(), 0);
    }

    // Create card for corporate
    {
        let card = crate::models::card::Card {
            kit_no: kit_no.clone(),
            entity_id: entity_id.clone(),
            card_number: crate::services::id_gen::generate_card_number(),
            card_type: "VIRTUAL".to_string(),
            card_category: "PREPAID".to_string(),
            status: crate::models::card::CardStatus::Active,
            network_type: "VISA".to_string(),
            expiry_date: crate::services::id_gen::generate_expiry(),
            physical_card_requested: false,
            created_at: now,
        };
        let mut cards = state.cards.write().unwrap();
        cards.insert(kit_no, card);
    }

    log::info!("Corporate registered: entity_id={}", entity_id);

    HttpResponse::Ok().json(ApiResponse::success(RegisterCorporateResponse {
        entity_id,
    }))
}
