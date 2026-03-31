use serde::{Deserialize, Serialize};

use super::card::AddressDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Corporate {
    pub entity_id: String,
    pub entity_type: String,
    pub business_type: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub kit_no: String,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCorporateRequest {
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub business_type: Option<String>,
    pub business_id: Option<String>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub special_date: Option<String>,
    pub kit_no: Option<String>,
    pub contact_no: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub pincode: Option<String>,
    pub id_type: Option<String>,
    pub id_number: Option<String>,
    pub countryof_issue: Option<String>,
    pub dependent: Option<bool>,
    pub id_expiry: Option<String>,
    pub kyc_ref_no: Option<String>,
    pub kyc_status: Option<String>,
    pub country_code: Option<String>,
    pub address_dto: Option<AddressDto>,
    pub contact_no1: Option<String>,
    pub email_address1: Option<String>,
    pub email_address2: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCorporateResponse {
    pub entity_id: String,
}
