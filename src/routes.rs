use actix_web::web;

use crate::handlers::{card, kyc, registration, transaction};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // KYC endpoints
        .service(
            web::scope("/kyc")
                .route("/customer/generate/otp", web::post().to(kyc::generate_otp))
                .route("/v2/register", web::post().to(kyc::register_customer)),
        )
        // Yappay endpoints
        .service(
            web::scope("/Yappay")
                // Transaction manager
                .service(
                    web::scope("/txn-manager")
                        .route("/create", web::post().to(transaction::create_transaction))
                        .route(
                            "/fetch/{extTrxId}",
                            web::get().to(transaction::fetch_by_ext_id),
                        )
                        .route(
                            "/fetch/success/entity/{entityId}",
                            web::get().to(transaction::fetch_by_entity),
                        ),
                )
                // Business entity manager
                .service(
                    web::scope("/business-entity-manager")
                        .route("/addCard", web::post().to(card::add_card))
                        .route(
                            "/fetchbalance/{entityId}",
                            web::get().to(card::fetch_balance),
                        )
                        .route("/v3/getCardList", web::post().to(card::get_card_list))
                        .route("/block", web::post().to(card::lock_unlock_block))
                        .route("/replaceCard", web::post().to(card::replace_card))
                        .route(
                            "/requestPhysicalCard",
                            web::post().to(card::request_physical_card),
                        )
                        .route("/setPreferences", web::post().to(card::set_preferences))
                        .route("/fetchPreference", web::post().to(card::fetch_preference)),
                )
                // Registration manager
                .service(
                    web::scope("/registration-manager")
                        .route("/register", web::post().to(registration::register_corporate)),
                ),
        );
}
