use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::{Error, HttpResponse, body::EitherBody};
use std::future::{Ready, ready, Future};
use std::pin::Pin;
use std::sync::Arc;

use crate::models::envelope::ApiResponse;
use crate::models::error;

pub struct AuthValidator {
    pub valid_tokens: Arc<Vec<String>>,
    pub valid_tenants: Arc<Vec<String>>,
}

impl<S, B> Transform<S, ServiceRequest> for AuthValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthValidatorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthValidatorMiddleware {
            service,
            valid_tokens: self.valid_tokens.clone(),
            valid_tenants: self.valid_tenants.clone(),
        }))
    }
}

pub struct AuthValidatorMiddleware<S> {
    service: S,
    valid_tokens: Arc<Vec<String>>,
    valid_tenants: Arc<Vec<String>>,
}

impl<S, B> Service<ServiceRequest> for AuthValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let has_valid_token = match &auth_header {
            Some(token) => self.valid_tokens.iter().any(|t| token == t || token == &format!("Bearer {}", t)),
            None => false,
        };

        // Check TENANT header
        let tenant_header = req
            .headers()
            .get("TENANT")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let has_valid_tenant = match &tenant_header {
            Some(tenant) => self.valid_tenants.iter().any(|t| t == tenant),
            None => false,
        };

        if !has_valid_token {
            let err = error::auth_failed();
            let response = HttpResponse::Unauthorized()
                .json(ApiResponse::<serde_json::Value>::error(err));
            let srv_response = req.into_response(response).map_into_right_body();
            return Box::pin(async move { Ok(srv_response) });
        }

        if !has_valid_tenant {
            let err = error::ErrorDetail::new("Y104", "Invalid or missing TENANT header");
            let response = HttpResponse::BadRequest()
                .json(ApiResponse::<serde_json::Value>::error(err));
            let srv_response = req.into_response(response).map_into_right_body();
            return Box::pin(async move { Ok(srv_response) });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}
