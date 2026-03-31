use rand::Rng;

pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(100_000..999_999))
}

pub fn generate_token() -> String {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    // Produce a base64-like token similar to M2P's format
    let bytes = id.as_bytes();
    let mut out = String::new();
    for b in bytes {
        out.push_str(&format!("{:02X}", b));
    }
    format!("{}=", &out[..32])
}

pub fn generate_txn_id() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100_000_000..999_999_999)
}

pub fn generate_kit_no() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{:012}",
        rng.gen_range(100_000_000_000_i64..999_999_999_999_i64)
    )
}

pub fn generate_card_number() -> String {
    let mut rng = rand::thread_rng();
    let last4: u16 = rng.gen_range(1000..9999);
    format!("4804XXXXXXXX{}", last4)
}

pub fn generate_expiry() -> String {
    let now = chrono::Utc::now();
    let expiry = now + chrono::Duration::days(365 * 3);
    expiry.format("%m%y").to_string()
}

pub fn generate_bank_tid() -> String {
    let mut rng = rand::thread_rng();
    format!("{}", rng.gen_range(100_000_000..999_999_999))
}
