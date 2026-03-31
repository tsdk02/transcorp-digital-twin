use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub entity_id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: Option<String>,
    pub mobile: String,
    pub email: Option<String>,
    pub token: String,
    pub kyc_status: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct OtpRecord {
    pub otp: String,
    pub entity_id: String,
    pub mobile: String,
    pub expires_at: i64,
}

// --- Request/Response types for KYC ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOtpRequest {
    pub entity_id: Option<String>,
    pub mobile_number: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOtpResponse {
    pub success: bool,
    pub entity_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCustomerRequest {
    pub entity_id: Option<String>,
    pub otp: Option<String>,
    pub channel_name: Option<String>,
    pub entity_type: Option<String>,
    pub business_type: Option<String>,
    pub business_id: Option<String>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    #[serde(default)]
    pub is_nri_customer: Option<bool>,
    #[serde(default)]
    pub is_minor: Option<bool>,
    #[serde(default)]
    pub is_dependant: Option<bool>,
    pub marital_status: Option<String>,
    pub country_code: Option<String>,
    pub employment_industry: Option<String>,
    pub employment_type: Option<String>,
    pub plastic_code: Option<String>,
    pub kit_info: Option<Vec<KitInfo>>,
    pub address_info: Option<Vec<AddressInfo>>,
    pub communication_info: Option<Vec<CommunicationInfo>>,
    pub kyc_info: Option<Vec<KycInfo>>,
    pub date_info: Option<Vec<DateInfo>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KitInfo {
    pub kit_no: Option<String>,
    pub card_type: Option<String>,
    pub card_category: Option<String>,
    pub card_reg_status: Option<String>,
    pub alias_name: Option<String>,
    pub fourth_line: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddressInfo {
    pub address_category: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub pin_code: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationInfo {
    pub contact_no: Option<String>,
    pub email_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KycInfo {
    pub document_type: Option<String>,
    pub document_no: Option<String>,
    pub document_expiry: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateInfo {
    pub date_type: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCustomerResponse {
    pub entity_id: String,
    pub kit_no: String,
    pub token: String,
    pub valid: bool,
}
