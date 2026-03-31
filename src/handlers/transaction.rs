use actix_web::{web, HttpResponse};

use crate::models::envelope::ApiResponse;
use crate::models::error;
use crate::models::transaction::*;
use crate::services::{balance, id_gen};
use crate::state::AppState;

pub async fn create_transaction(
    body: web::Json<CreateTransactionRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let to_entity_id = match &body.to_entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("toEntityId").to_response(),
    };

    let from_entity_id = match &body.from_entity_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("fromEntityId").to_response(),
    };

    let amount_value = match &body.amount {
        Some(v) => v.clone(),
        None => return error::missing_field("amount").to_response(),
    };

    let amount_paise = match balance::parse_amount_to_paise(&amount_value) {
        Some(p) if p > 0 => p,
        _ => {
            return error::ErrorDetail::new("Y512", "Amount Must be greater than zero").to_response()
        }
    };

    let txn_type = match &body.transaction_type {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return error::missing_field("transactionType").to_response(),
    };

    let ext_txn_id = match &body.external_transaction_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return error::missing_field("externalTransactionId").to_response(),
    };

    // Check for duplicate external transaction id
    {
        let txns = state.transactions.read().unwrap();
        if txns.contains_key(&ext_txn_id) {
            return error::ErrorDetail::new("Y2004", "Transaction repeated within allowable time")
                .to_response();
        }
    }

    let now = chrono::Utc::now().timestamp_millis();
    let tx_id = id_gen::generate_txn_id();

    // Process based on transaction type
    let (credit_debit_type, new_balance) = match txn_type.as_str() {
        "M2C" => {
            // Merchant/Corporate to Customer load — only credit the recipient
            match balance::credit(&state.balances, &to_entity_id, amount_paise) {
                Ok(b) => ("CREDIT".to_string(), Some(balance::paise_to_amount(b))),
                Err(e) => return e.to_response(),
            }
        }
        "C2C" => {
            // Customer to Customer — debit sender, credit receiver
            if let Err(e) = balance::debit(&state.balances, &from_entity_id, amount_paise) {
                return e.to_response();
            }
            match balance::credit(&state.balances, &to_entity_id, amount_paise) {
                Ok(b) => ("CREDIT".to_string(), Some(balance::paise_to_amount(b))),
                Err(e) => return e.to_response(),
            }
        }
        "B2C" => {
            // Business to Consumer — debit corporate, credit customer
            if let Err(e) = balance::debit(&state.balances, &from_entity_id, amount_paise) {
                return e.to_response();
            }
            match balance::credit(&state.balances, &to_entity_id, amount_paise) {
                Ok(b) => ("CREDIT".to_string(), Some(balance::paise_to_amount(b))),
                Err(e) => return e.to_response(),
            }
        }
        "C2B" => {
            // Consumer to Business — debit customer, credit corporate
            if let Err(e) = balance::debit(&state.balances, &from_entity_id, amount_paise) {
                return e.to_response();
            }
            match balance::credit(&state.balances, &to_entity_id, amount_paise) {
                Ok(b) => ("DEBIT".to_string(), Some(balance::paise_to_amount(b))),
                Err(e) => return e.to_response(),
            }
        }
        _ => {
            return error::ErrorDetail::new("Y401", "Invalid flow").to_response();
        }
    };

    let amount_display = balance::paise_to_amount(amount_paise);

    let txn = Transaction {
        tx_ref: tx_id,
        amount: format!("{:.1}", amount_display),
        balance: new_balance,
        transaction_type: txn_type.clone(),
        credit_debit_type: Some(credit_debit_type),
        time: now,
        business_id: body.business.clone(),
        beneficiary_name: None,
        beneficiary_type: None,
        beneficiary_id: Some(to_entity_id.clone()),
        description: body.description.clone(),
        other_party_name: None,
        other_party_id: Some(to_entity_id.clone()),
        txn_origin: body.transaction_origin.clone(),
        transaction_status: "PAYMENT_SUCCESS".to_string(),
        status: None,
        your_wallet: Some("GENERAL".to_string()),
        beneficiary_wallet: Some(String::new()),
        external_transaction_id: ext_txn_id.clone(),
        retrival_reference_no: None,
        auth_code: None,
        bill_ref_no: None,
        bank_tid: Some(id_gen::generate_bank_tid()),
    };

    {
        let mut txns = state.transactions.write().unwrap();
        txns.insert(ext_txn_id.clone(), txn);
    }

    // Fire webhook notification asynchronously
    if let Some(webhook_url) = &state.partner_webhook_url {
        let url = webhook_url.clone();
        let notification = WebhookNotification {
            entity_id: Some(to_entity_id),
            txn_ref_no: tx_id.to_string(),
            mobile_no: None,
            txn_amt: format!("{:.1}", amount_display),
            merchant_id: None,
            merchant_name: None,
            merchant_location: None,
            txn_date: chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
            balance: new_balance.map(|b| format!("{:.2}", b)).unwrap_or_default(),
            transaction_type: txn_type,
            sender_account: None,
            sender_name: None,
            prod_type: Some("GENERAL".to_string()),
            txn_status: "PAYMENT_SUCCESS".to_string(),
            txn_desc: body.description.clone(),
            mcc: None,
            ext_txn_id,
            channel: body.transaction_origin.clone(),
            cur_code: Some("356".to_string()),
            proxy_card_no: None,
            retrieval_ref_no: None,
            terminal_id: None,
            trace_no: None,
            txn_currency: Some("INR".to_string()),
            txn_origin: body.transaction_origin.clone(),
            acquirer_id: None,
            network: None,
            auth_code: None,
            transaction_fees: Some("0.0".to_string()),
            description: body.description.clone(),
        };

        tokio::spawn(async move {
            crate::handlers::webhook::send_notification(&url, &notification).await;
        });
    }

    log::info!("Transaction created: tx_id={}", tx_id);

    HttpResponse::Ok().json(ApiResponse::success(CreateTransactionResponse { tx_id }))
}

pub async fn fetch_by_ext_id(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let ext_txn_id = path.into_inner();

    let txns = state.transactions.read().unwrap();
    match txns.get(&ext_txn_id) {
        Some(txn) => HttpResponse::Ok().json(ApiResponse::success(FetchTransactionResponse {
            transaction: txn.clone(),
            balance: None,
        })),
        None => error::invalid_txn_id(&ext_txn_id).to_response(),
    }
}

pub async fn fetch_by_entity(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let entity_id = path.into_inner();

    let txns = state.transactions.read().unwrap();
    let entity_txns: Vec<&Transaction> = txns
        .values()
        .filter(|t| {
            t.beneficiary_id.as_deref() == Some(&entity_id)
                || t.other_party_id.as_deref() == Some(&entity_id)
        })
        .filter(|t| t.transaction_status == "PAYMENT_SUCCESS")
        .collect();

    if entity_txns.is_empty() {
        // Return empty result, not an error (per spec)
        return HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "transaction": serde_json::Value::Null,
            "balance": serde_json::Value::Null
        })));
    }

    // Return most recent transaction
    let latest = entity_txns
        .iter()
        .max_by_key(|t| t.time)
        .unwrap();

    HttpResponse::Ok().json(ApiResponse::success(FetchTransactionResponse {
        transaction: (*latest).clone(),
        balance: None,
    }))
}
