use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

use crate::utils::jwt::{validate_token, Claims};

// ─── Middleware factory ───────────────────────────────────────────────────────

pub struct JwtAuth {
    /// If Some, only this role may proceed; if None any valid token is accepted.
    pub required_role: Option<String>,
}

impl JwtAuth {
    pub fn new() -> Self {
        Self { required_role: None }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service,
            required_role: self.required_role.clone(),
        })
    }
}

// ─── Middleware service ───────────────────────────────────────────────────────

pub struct JwtAuthMiddleware<S> {
    service: S,
    required_role: Option<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let required_role = self.required_role.clone();

        // Extract Bearer token
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|t| t.to_string());

        let token = match token {
            Some(t) => t,
            None => {
                let (req, _payload) = req.into_parts();
                let resp = HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": "Missing Authorization header"}))
                    .map_into_right_body();
                return Box::pin(async move { Ok(ServiceResponse::new(req, resp)) });
            }
        };

        match validate_token(&token) {
            Ok(data) => {
                let claims: Claims = data.claims;

                // Role check
                if let Some(role) = &required_role {
                    if &claims.role != role {
                        let (req, _payload) = req.into_parts();
                        let resp = HttpResponse::Forbidden()
                            .json(serde_json::json!({"error": "Insufficient permissions"}))
                            .map_into_right_body();
                        return Box::pin(async move { Ok(ServiceResponse::new(req, resp)) });
                    }
                }

                // Attach claims to request extensions for handlers to read
                req.extensions_mut().insert(claims);

                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
            Err(e) => {
                let (req, _payload) = req.into_parts();
                let resp = HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": format!("Invalid token: {e}")}))
                    .map_into_right_body();
                Box::pin(async move { Ok(ServiceResponse::new(req, resp)) })
            }
        }
    }
}
