use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "LOCKED")]
    Locked,
    #[serde(rename = "ALLOCATED")]
    Allocated,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "REPLACED")]
    Replaced,
    #[serde(rename = "UNACTIVATED")]
    Unactivated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub kit_no: String,
    pub entity_id: String,
    pub card_number: String,
    pub card_type: String,
    pub card_category: String,
    pub status: CardStatus,
    pub network_type: String,
    pub expiry_date: String,
    pub physical_card_requested: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPreferences {
    pub entity_id: String,
    pub atm: Option<bool>,
    pub pos: Option<bool>,
    pub ecom: Option<bool>,
    pub dcc: Option<bool>,
    pub contactless: Option<bool>,
    pub international: Option<bool>,
    pub limit_config: Option<LimitConfig>,
    pub overall_limit_config: Option<OverallLimitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitConfig {
    pub txn_type: Option<String>,
    pub daily_limit_value: Option<String>,
    pub daily_limit_cnt: Option<String>,
    pub max_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallLimitConfig {
    pub daily_limit_value: Option<String>,
    pub daily_limit_cnt: Option<String>,
}

// --- Request types ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCardRequest {
    pub entity_id: Option<String>,
    pub business_type: Option<String>,
    pub card_type: Option<String>,
    pub kit_no: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCardListRequest {
    pub entity_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCardListResponse {
    pub card_list: Vec<String>,
    pub kit_list: Vec<String>,
    pub expiry_date_list: Vec<String>,
    pub card_status_list: Vec<String>,
    pub card_type_list: Vec<String>,
    pub network_type_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockRequest {
    pub entity_id: Option<String>,
    pub kit_no: Option<String>,
    pub flag: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCardRequest {
    pub entity_id: Option<String>,
    pub old_kit_no: Option<String>,
    pub new_kit_no: Option<String>,
    pub business_type: Option<String>,
    pub card_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPhysicalCardRequest {
    pub entity_id: Option<String>,
    pub kit_no: Option<String>,
    pub address_dto: Option<AddressDto>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddressDto {
    pub address: Option<Vec<DeliveryAddress>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAddress {
    pub title: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub fourth_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub pin_code: Option<String>,
    pub alias_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPreferencesRequest {
    pub entity_id: Option<String>,
    pub status: Option<String>,
    pub atm: Option<bool>,
    pub pos: Option<bool>,
    pub ecom: Option<bool>,
    pub dcc: Option<bool>,
    pub contactless: Option<bool>,
    pub international: Option<bool>,
    pub limit_config: Option<LimitConfig>,
    pub overall_limit_config: Option<OverallLimitConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchPreferenceRequest {
    pub entity_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchPreferenceResponse {
    pub atm: Option<bool>,
    pub pos: Option<bool>,
    pub ecom: Option<bool>,
    pub dcc: Option<bool>,
    pub contactless: Option<bool>,
    pub international: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePreferenceExternalRequest {
    pub entity_id: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub pref_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PciCardDetailsRequest {
    pub token: Option<String>,
    pub kit_no: Option<String>,
    pub entity_id: Option<String>,
    pub app_guid: Option<String>,
    pub business: Option<String>,
    pub callback_url: Option<String>,
    pub dob: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResult {
    pub success: bool,
}
