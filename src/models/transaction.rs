use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub tx_ref: i64,
    pub amount: String,
    pub balance: Option<f64>,
    pub transaction_type: String,
    #[serde(rename = "type")]
    pub credit_debit_type: Option<String>,
    pub time: i64,
    pub business_id: Option<String>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_type: Option<String>,
    pub beneficiary_id: Option<String>,
    pub description: Option<String>,
    pub other_party_name: Option<String>,
    pub other_party_id: Option<String>,
    pub txn_origin: Option<String>,
    pub transaction_status: String,
    pub status: Option<String>,
    pub your_wallet: Option<String>,
    pub beneficiary_wallet: Option<String>,
    pub external_transaction_id: String,
    pub retrival_reference_no: Option<String>,
    pub auth_code: Option<String>,
    pub bill_ref_no: Option<String>,
    pub bank_tid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    pub to_entity_id: Option<String>,
    pub from_entity_id: Option<String>,
    pub yapcode: Option<String>,
    pub product_id: Option<String>,
    pub description: Option<String>,
    pub amount: Option<serde_json::Value>,
    pub transaction_type: Option<String>,
    pub transaction_origin: Option<String>,
    pub business: Option<String>,
    pub business_entity_id: Option<String>,
    pub business_type: Option<String>,
    pub external_transaction_id: Option<String>,
    pub from_product_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionResponse {
    pub tx_id: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchTransactionResponse {
    pub transaction: Transaction,
    pub balance: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchTxnPagingQuery {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

// Webhook notification payload
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookNotification {
    pub entity_id: Option<String>,
    pub txn_ref_no: String,
    pub mobile_no: Option<String>,
    pub txn_amt: String,
    pub merchant_id: Option<String>,
    pub merchant_name: Option<String>,
    pub merchant_location: Option<String>,
    pub txn_date: String,
    pub balance: String,
    pub transaction_type: String,
    pub sender_account: Option<String>,
    pub sender_name: Option<String>,
    pub prod_type: Option<String>,
    pub txn_status: String,
    pub txn_desc: Option<String>,
    pub mcc: Option<String>,
    pub ext_txn_id: String,
    pub channel: Option<String>,
    pub cur_code: Option<String>,
    pub proxy_card_no: Option<String>,
    pub retrieval_ref_no: Option<String>,
    pub terminal_id: Option<String>,
    pub trace_no: Option<String>,
    pub txn_currency: Option<String>,
    pub txn_origin: Option<String>,
    pub acquirer_id: Option<String>,
    pub network: Option<String>,
    pub auth_code: Option<String>,
    pub transaction_fees: Option<String>,
    pub description: Option<String>,
}
