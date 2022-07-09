use std::pin::Pin;

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};

use crate::jwt::handlers::decode_auth_token;

pub struct AuthTokenService;

impl<S, B> Transform<S, ServiceRequest> for AuthTokenService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthTokenMiddleware { service })
    }
}

pub struct AuthTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let unauthorized_res = HttpResponse::Unauthorized().finish().map_into_right_body();

        match req.headers().get("Authorization") {
            Some(header) => {
                if let Ok(header) = header.to_str() {
                    if header.starts_with("Bearer") {
                        let header_vec: Vec<&str> = header.split_whitespace().collect();
                        let token = header_vec.get(1);

                        if let Some(token) = token {
                            if let Ok(payload) = decode_auth_token(token) {
                                req.guard_ctx().req_data_mut().insert(payload);

                                let res_fut = self.service.call(req);

                                return Box::pin(async {
                                    let ok_res = res_fut.await;
                                    ok_res.map(ServiceResponse::map_into_left_body)
                                });
                            }

                            return Box::pin(async { Ok(req.into_response(unauthorized_res)) });
                        }
                        return Box::pin(async { Ok(req.into_response(unauthorized_res)) });
                    }
                }
                Box::pin(async { Ok(req.into_response(unauthorized_res)) })
            }
            None => Box::pin(async { Ok(req.into_response(unauthorized_res)) }),
        }
    }
}
