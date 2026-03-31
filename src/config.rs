use serde::Deserialize;

const AUTH_FILE: &str = "auth.json";

pub struct AppConfig {
    pub port: u16,
    pub partner_webhook_url: Option<String>,
    pub valid_auth_tokens: Vec<String>,
    pub valid_tenants: Vec<String>,
}

#[derive(Deserialize)]
struct AuthConfig {
    valid_tokens: Vec<String>,
    valid_tenants: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let (tokens, tenants) = Self::load_auth_config();

        Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8081),
            partner_webhook_url: std::env::var("PARTNER_WEBHOOK_URL").ok(),
            valid_auth_tokens: tokens,
            valid_tenants: tenants,
        }
    }

    fn load_auth_config() -> (Vec<String>, Vec<String>) {
        match std::fs::read_to_string(AUTH_FILE) {
            Ok(data) => match serde_json::from_str::<AuthConfig>(&data) {
                Ok(config) => {
                    log::info!(
                        "Loaded {} tokens and {} tenants from {}",
                        config.valid_tokens.len(),
                        config.valid_tenants.len(),
                        AUTH_FILE
                    );
                    (config.valid_tokens, config.valid_tenants)
                }
                Err(e) => {
                    log::error!("Failed to parse {}: {}", AUTH_FILE, e);
                    Self::defaults()
                }
            },
            Err(e) => {
                log::warn!("No {} found ({}), using defaults", AUTH_FILE, e);
                Self::defaults()
            }
        }
    }

    fn defaults() -> (Vec<String>, Vec<String>) {
        (
            vec!["test-token".to_string()],
            vec!["BUSINESS".to_string(), "SANDBOXTEST".to_string()],
        )
    }
}
