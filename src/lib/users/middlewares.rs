use std::pin::Pin;

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceFactory, ServiceRequest, ServiceResponse},
    http::StatusCode,
    Error,
};
use futures::Future;

use crate::jwt::handlers::decode_auth_token;

use super::payloads::ErrorResponse;

pub fn validate_user_token<S, B>(
    req: ServiceRequest,
    srv: &<dyn ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<B>>,
        Error = Error,
        InitError = (),
        Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<EitherBody<B>>, Error>>>>,
        Service = S,
    > as ServiceFactory<ServiceRequest>>::Service,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<EitherBody<B>>, Error>>>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    let unauthorized_res = ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        String::from("Unauthenticated request"),
    )
    .map_into_right_body();

    match req.headers().get("Authorization") {
        Some(header) => {
            if let Ok(header) = header.to_str() {
                if header.starts_with("Bearer") {
                    let header_vec: Vec<&str> = header.split_whitespace().collect();
                    let token = header_vec.get(1);

                    if let Some(token) = token {
                        if let Ok(payload) = decode_auth_token(token) {
                            req.guard_ctx().req_data_mut().insert(payload);

                            let res_fut = srv.call(req);

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
