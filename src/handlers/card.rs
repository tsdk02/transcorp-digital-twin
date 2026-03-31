use actix_web::{web, HttpResponse};

use crate::models::card::*;
use crate::models::envelope::ApiResponse;
use crate::models::error;
use crate::services::{balance, id_gen};
use crate::state::AppState;

pub async fn add_card(body: web::Json<AddCardRequest>, state: web::Data<AppState>) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    // Verify entity exists (customer or corporate)
    {
        let customers = state.customers.read().unwrap();
        let corporates = state.corporates.read().unwrap();
        if !customers.contains_key(&entity_id) && !corporates.contains_key(&entity_id) {
            return error::entity_not_found(&entity_id).to_response();
        }
    }

    let kit_no = body.kit_no.clone().unwrap_or_else(id_gen::generate_kit_no);

    let card_type = match &body.card_type {
        Some(ct) => match ct.as_str() {
            "P" => "PHYSICAL".to_string(),
            "V" => "VIRTUAL".to_string(),
            other => other.to_string(),
        },
        None => "VIRTUAL".to_string(),
    };

    let now = chrono::Utc::now().timestamp();
    let card = Card {
        kit_no: kit_no.clone(),
        entity_id: entity_id.clone(),
        card_number: id_gen::generate_card_number(),
        card_type,
        card_category: "PREPAID".to_string(),
        status: CardStatus::Active,
        network_type: "VISA".to_string(),
        expiry_date: id_gen::generate_expiry(),
        physical_card_requested: false,
        created_at: now,
    };

    {
        let mut cards = state.cards.write().unwrap();
        cards.insert(kit_no, card);
    }

    HttpResponse::Ok().json(ApiResponse::success(SuccessResult { success: true }))
}

pub async fn fetch_balance(path: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let entity_id = path.into_inner();

    let balances = state.balances.read().unwrap();
    match balances.get(&entity_id) {
        Some(&balance_paise) => {
            let amount = balance::paise_to_amount(balance_paise);
            HttpResponse::Ok().json(ApiResponse::success(vec![serde_json::json!({
                "entityId": entity_id,
                "productId": "GENERAL",
                "balance": format!("{:.1}", amount),
                "lienBalance": "0.0"
            })]))
        }
        None => error::entity_not_found(&entity_id).to_response(),
    }
}

pub async fn get_card_list(
    body: web::Json<GetCardListRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let cards = state.cards.read().unwrap();
    let entity_cards: Vec<&Card> = cards
        .values()
        .filter(|c| c.entity_id == entity_id)
        .collect();

    if entity_cards.is_empty() {
        return error::no_data().to_response();
    }

    let response = GetCardListResponse {
        card_list: entity_cards.iter().map(|c| c.card_number.clone()).collect(),
        kit_list: entity_cards.iter().map(|c| c.kit_no.clone()).collect(),
        expiry_date_list: entity_cards.iter().map(|c| c.expiry_date.clone()).collect(),
        card_status_list: entity_cards
            .iter()
            .map(|c| {
                serde_json::to_value(&c.status)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect(),
        card_type_list: entity_cards.iter().map(|c| c.card_type.clone()).collect(),
        network_type_list: entity_cards
            .iter()
            .map(|c| c.network_type.clone())
            .collect(),
    };

    HttpResponse::Ok().json(ApiResponse::success(response))
}

pub async fn lock_unlock_block(
    body: web::Json<BlockRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let kit_no = match &body.kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("kitNo").to_response(),
    };

    let flag = match &body.flag {
        Some(f) if !f.is_empty() => f.clone(),
        _ => return error::missing_field("flag").to_response(),
    };

    let mut cards = state.cards.write().unwrap();
    let card = match cards.get_mut(&kit_no) {
        Some(c) => c,
        None => return error::kit_not_found().to_response(),
    };

    if card.entity_id != entity_id {
        return error::kit_entity_mismatch().to_response();
    }

    match flag.as_str() {
        "L" => {
            if card.status == CardStatus::Blocked {
                return error::ErrorDetail::new("Y224", "The Entity is blocked from transaction")
                    .to_response();
            }
            card.status = CardStatus::Locked;
        }
        "UL" => {
            if card.status == CardStatus::Blocked {
                return error::ErrorDetail::new("Y224", "The Entity is blocked from transaction")
                    .to_response();
            }
            card.status = CardStatus::Active;
        }
        "BL" => {
            card.status = CardStatus::Blocked;
        }
        _ => return error::invalid_flag().to_response(),
    }

    HttpResponse::Ok().json(ApiResponse::success(SuccessResult { success: true }))
}

pub async fn replace_card(
    body: web::Json<ReplaceCardRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let old_kit_no = match &body.old_kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("oldKitNo").to_response(),
    };

    let mut cards = state.cards.write().unwrap();

    // Check old card exists and is blocked
    let old_card = match cards.get(&old_kit_no) {
        Some(c) => c.clone(),
        None => return error::kit_not_found().to_response(),
    };

    if old_card.entity_id != entity_id {
        return error::kit_entity_mismatch().to_response();
    }

    if old_card.status != CardStatus::Blocked {
        return error::block_old_card().to_response();
    }

    // Mark old card as replaced
    if let Some(old) = cards.get_mut(&old_kit_no) {
        old.status = CardStatus::Replaced;
    }

    // Create new card
    let new_kit_no = body
        .new_kit_no
        .clone()
        .unwrap_or_else(id_gen::generate_kit_no);

    let card_type = body.card_type.clone().unwrap_or(old_card.card_type.clone());

    let now = chrono::Utc::now().timestamp();
    let new_card = Card {
        kit_no: new_kit_no.clone(),
        entity_id,
        card_number: id_gen::generate_card_number(),
        card_type,
        card_category: old_card.card_category.clone(),
        status: CardStatus::Active,
        network_type: old_card.network_type.clone(),
        expiry_date: id_gen::generate_expiry(),
        physical_card_requested: false,
        created_at: now,
    };

    cards.insert(new_kit_no, new_card);

    HttpResponse::Ok().json(ApiResponse::success("success"))
}

pub async fn request_physical_card(
    body: web::Json<RequestPhysicalCardRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let kit_no = match &body.kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("kitNo").to_response(),
    };

    let mut cards = state.cards.write().unwrap();
    let card = match cards.get_mut(&kit_no) {
        Some(c) => c,
        None => return error::kit_not_found().to_response(),
    };

    if card.entity_id != entity_id {
        return error::kit_entity_mismatch().to_response();
    }

    card.physical_card_requested = true;

    HttpResponse::Ok().json(ApiResponse::success(true))
}

pub async fn set_preferences(
    body: web::Json<SetPreferencesRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    // Verify entity exists
    {
        let customers = state.customers.read().unwrap();
        let corporates = state.corporates.read().unwrap();
        if !customers.contains_key(&entity_id) && !corporates.contains_key(&entity_id) {
            return error::entity_not_found(&entity_id).to_response();
        }
    }

    let mut prefs = state.preferences.write().unwrap();
    let existing = prefs
        .entry(entity_id.clone())
        .or_insert_with(|| CardPreferences {
            entity_id: entity_id.clone(),
            ..Default::default()
        });

    if let Some(v) = body.atm {
        existing.atm = Some(v);
    }
    if let Some(v) = body.pos {
        existing.pos = Some(v);
    }
    if let Some(v) = body.ecom {
        existing.ecom = Some(v);
    }
    if let Some(v) = body.dcc {
        existing.dcc = Some(v);
    }
    if let Some(v) = body.contactless {
        existing.contactless = Some(v);
    }
    if let Some(v) = body.international {
        existing.international = Some(v);
    }
    if body.limit_config.is_some() {
        existing.limit_config = body.limit_config.clone();
    }
    if body.overall_limit_config.is_some() {
        existing.overall_limit_config = body.overall_limit_config.clone();
    }

    HttpResponse::Ok().json(ApiResponse::success(true))
}

pub async fn update_preference_external(
    body: web::Json<UpdatePreferenceExternalRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let status = match &body.status {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return error::missing_field("status").to_response(),
    };

    let pref_type = match &body.pref_type {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return error::missing_field("type").to_response(),
    };

    // Verify entity exists
    {
        let customers = state.customers.read().unwrap();
        let corporates = state.corporates.read().unwrap();
        if !customers.contains_key(&entity_id) && !corporates.contains_key(&entity_id) {
            return error::entity_not_found(&entity_id).to_response();
        }
    }

    let enabled = status == "ALLOWED";

    let mut prefs = state.preferences.write().unwrap();
    let existing = prefs
        .entry(entity_id.clone())
        .or_insert_with(|| CardPreferences {
            entity_id: entity_id.clone(),
            ..Default::default()
        });

    match pref_type.as_str() {
        "ECOM" => existing.ecom = Some(enabled),
        "POS" => existing.pos = Some(enabled),
        "ATM" => existing.atm = Some(enabled),
        "CONTACTLESS" => existing.contactless = Some(enabled),
        "INTERNATIONAL" => existing.international = Some(enabled),
        "DCC" => existing.dcc = Some(enabled),
        _ => {}
    }

    HttpResponse::Ok().json(ApiResponse::success(SuccessResult { success: true }))
}

pub async fn pci_card_details(
    body: web::Json<PciCardDetailsRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let kit_no = match &body.kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("kitNo").to_response(),
    };

    // Verify card exists
    {
        let cards = state.cards.read().unwrap();
        match cards.get(&kit_no) {
            Some(c) if c.entity_id == entity_id => {}
            Some(_) => return error::kit_entity_mismatch().to_response(),
            None => return error::kit_not_found().to_response(),
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "result": {
            "url": format!("https://mock-pci.example.com/card/{}", kit_no)
        },
        "exception": null,
        "pagination": null
    }))
}

pub async fn set_pin(
    body: web::Json<PciCardDetailsRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let kit_no = match &body.kit_no {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return error::missing_field("kitNo").to_response(),
    };

    // Verify card exists
    {
        let cards = state.cards.read().unwrap();
        match cards.get(&kit_no) {
            Some(c) if c.entity_id == entity_id => {}
            Some(_) => return error::kit_entity_mismatch().to_response(),
            None => return error::kit_not_found().to_response(),
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "result": {
            "url": format!("https://mock-pci.example.com/setpin/{}", kit_no)
        },
        "exception": null,
        "pagination": null
    }))
}

pub async fn fetch_preference(
    body: web::Json<FetchPreferenceRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = match &body.entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("entityId").to_response(),
    };

    let prefs = state.preferences.read().unwrap();
    match prefs.get(&entity_id) {
        Some(p) => HttpResponse::Ok().json(ApiResponse::success(FetchPreferenceResponse {
            atm: p.atm,
            pos: p.pos,
            ecom: p.ecom,
            dcc: p.dcc,
            contactless: p.contactless,
            international: p.international,
        })),
        None => {
            // Return defaults if no preferences set
            HttpResponse::Ok().json(ApiResponse::success(FetchPreferenceResponse {
                atm: Some(true),
                pos: Some(true),
                ecom: Some(true),
                dcc: Some(false),
                contactless: Some(true),
                international: Some(false),
            }))
        }
    }
}
