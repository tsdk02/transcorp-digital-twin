use crate::models::transaction::WebhookNotification;

pub async fn send_notification(url: &str, payload: &WebhookNotification) {
    let client = reqwest::Client::new();
    match client.post(url).json(payload).send().await {
        Ok(resp) => {
            log::info!(
                "Webhook notification sent to {}: status={}",
                url,
                resp.status()
            );
        }
        Err(e) => {
            log::warn!("Webhook notification failed to {}: {}", url, e);
        }
    }
}
