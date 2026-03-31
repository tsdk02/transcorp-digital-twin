use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::error::{self, ErrorDetail};

/// Credit an entity's balance. Amount is in paise.
/// Creates the balance entry if it doesn't exist.
pub fn credit(
    balances: &RwLock<HashMap<String, i64>>,
    entity_id: &str,
    amount_paise: i64,
) -> Result<i64, ErrorDetail> {
    let mut map = balances.write().unwrap();
    let balance = map.entry(entity_id.to_string()).or_insert(0);
    *balance += amount_paise;
    Ok(*balance)
}

/// Debit an entity's balance. Amount is in paise.
/// Returns error if insufficient balance.
pub fn debit(
    balances: &RwLock<HashMap<String, i64>>,
    entity_id: &str,
    amount_paise: i64,
) -> Result<i64, ErrorDetail> {
    let mut map = balances.write().unwrap();
    let balance = map.entry(entity_id.to_string()).or_insert(0);
    if *balance < amount_paise {
        return Err(error::insufficient_balance());
    }
    *balance -= amount_paise;
    Ok(*balance)
}

/// Parse amount from JSON value (can be string or number) to paise (i64).
pub fn parse_amount_to_paise(value: &serde_json::Value) -> Option<i64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64().map(|f| (f * 100.0) as i64),
        serde_json::Value::String(s) => s.parse::<f64>().ok().map(|f| (f * 100.0) as i64),
        _ => None,
    }
}

/// Convert paise to display amount (f64).
pub fn paise_to_amount(paise: i64) -> f64 {
    paise as f64 / 100.0
}
