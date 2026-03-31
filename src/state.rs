use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::models::card::{Card, CardPreferences};
use crate::models::corporate::Corporate;
use crate::models::customer::{Customer, OtpRecord};
use crate::models::transaction::Transaction;

const SNAPSHOT_FILE: &str = "data/state.json";

pub struct AppState {
    pub customers: RwLock<HashMap<String, Customer>>,
    pub otps: RwLock<HashMap<String, OtpRecord>>,
    pub corporates: RwLock<HashMap<String, Corporate>>,
    pub cards: RwLock<HashMap<String, Card>>,
    pub transactions: RwLock<HashMap<String, Transaction>>,
    /// Balances stored in paise (integer cents) to avoid floating point issues.
    /// Keyed by entityId.
    pub balances: RwLock<HashMap<String, i64>>,
    pub preferences: RwLock<HashMap<String, CardPreferences>>,
    pub partner_webhook_url: Option<String>,
}

/// Serializable snapshot of all state (excludes OTPs — they're ephemeral).
#[derive(Serialize, Deserialize)]
struct Snapshot {
    customers: HashMap<String, Customer>,
    corporates: HashMap<String, Corporate>,
    cards: HashMap<String, Card>,
    transactions: HashMap<String, Transaction>,
    balances: HashMap<String, i64>,
    preferences: HashMap<String, CardPreferences>,
}

impl AppState {
    pub fn new(partner_webhook_url: Option<String>) -> Self {
        // Try loading from snapshot file
        if let Some(snapshot) = Self::load_snapshot() {
            log::info!("Loaded state from {}", SNAPSHOT_FILE);
            return Self {
                customers: RwLock::new(snapshot.customers),
                otps: RwLock::new(HashMap::new()),
                corporates: RwLock::new(snapshot.corporates),
                cards: RwLock::new(snapshot.cards),
                transactions: RwLock::new(snapshot.transactions),
                balances: RwLock::new(snapshot.balances),
                preferences: RwLock::new(snapshot.preferences),
                partner_webhook_url,
            };
        }

        Self {
            customers: RwLock::new(HashMap::new()),
            otps: RwLock::new(HashMap::new()),
            corporates: RwLock::new(HashMap::new()),
            cards: RwLock::new(HashMap::new()),
            transactions: RwLock::new(HashMap::new()),
            balances: RwLock::new(HashMap::new()),
            preferences: RwLock::new(HashMap::new()),
            partner_webhook_url,
        }
    }

    fn load_snapshot() -> Option<Snapshot> {
        let path = Path::new(SNAPSHOT_FILE);
        if !path.exists() {
            return None;
        }
        let data = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save_snapshot(&self) {
        let snapshot = Snapshot {
            customers: self.customers.read().unwrap().clone(),
            corporates: self.corporates.read().unwrap().clone(),
            cards: self.cards.read().unwrap().clone(),
            transactions: self.transactions.read().unwrap().clone(),
            balances: self.balances.read().unwrap().clone(),
            preferences: self.preferences.read().unwrap().clone(),
        };

        // Ensure data directory exists
        let _ = std::fs::create_dir_all("data");

        match serde_json::to_string_pretty(&snapshot) {
            Ok(json) => {
                if let Err(e) = std::fs::write(SNAPSHOT_FILE, json) {
                    log::error!("Failed to save state: {}", e);
                } else {
                    log::info!("State saved to {}", SNAPSHOT_FILE);
                }
            }
            Err(e) => log::error!("Failed to serialize state: {}", e),
        }
    }
}
