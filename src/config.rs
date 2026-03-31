pub struct AppConfig {
    pub port: u16,
    pub partner_webhook_url: Option<String>,
    pub valid_auth_tokens: Vec<String>,
    pub valid_tenants: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            partner_webhook_url: std::env::var("PARTNER_WEBHOOK_URL").ok(),
            valid_auth_tokens: std::env::var("VALID_AUTH_TOKENS")
                .unwrap_or_else(|_| "test-token".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            valid_tenants: std::env::var("VALID_TENANTS")
                .unwrap_or_else(|_| "BUSINESS,SANDBOXTEST".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}
