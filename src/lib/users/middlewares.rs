use std::{env, pin::Pin};

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceFactory, ServiceRequest, ServiceResponse},
    http::StatusCode,
    web, Error, HttpMessage,
};
use futures::Future;
use jsonwebtoken::TokenData;

use crate::{
    errors::ErrorResponse,
    jwt::{handlers::decode_auth_token, payload::AuthToken},
};

use super::{errors::GetSelfError, UserService};

pub fn validate_user_token<S, B>(
    req: ServiceRequest,
    service: &<dyn ServiceFactory<
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
    let unauthorized_res =
        ErrorResponse::new(StatusCode::UNAUTHORIZED, "Unauthorized request").map_into_right_body();

    match req.headers().get("Authorization") {
        Some(header) => {
            if let Ok(header) = header.to_str() {
                if header.starts_with("Bearer") {
                    let header_vec: Vec<&str> = header.split_whitespace().collect();
                    let token = header_vec.get(1);

                    if let Some(token) = token {
                        if let Ok(payload) = decode_auth_token(token) {
                            req.extensions_mut().insert(payload);

                            let res_fut = service.call(req);

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

pub fn validate_super_user<S, B>(
    req: ServiceRequest,
    service: &<dyn ServiceFactory<
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
    let forbidden_res =
        ErrorResponse::new(StatusCode::FORBIDDEN, "Not permitted to access resource")
            .map_into_right_body();
    let not_found_res =
        ErrorResponse::new(StatusCode::NOT_FOUND, "User not found").map_into_right_body();
    let internal_server_err_res =
        ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            .map_into_right_body();

    let (http_req, payload) = req.into_parts();
    let new_http_req = http_req.clone();
    let new_req = ServiceRequest::from_parts(http_req, payload);
    let res_fut = service.call(new_req);

    Box::pin(async move {
        let super_username = match env::var("SUPER_USER") {
            Ok(val) => val,
            Err(err) => {
                log::error!("{err}");
                return Ok(ServiceResponse::new(new_http_req, internal_server_err_res));
            }
        };

        let super_email = match env::var("SUPER_EMAIL") {
            Ok(val) => val,
            Err(err) => {
                log::error!("{err}");
                return Ok(ServiceResponse::new(new_http_req, internal_server_err_res));
            }
        };

        let user_service = new_http_req.app_data::<web::Data<UserService>>().unwrap();

        let AuthToken { id, .. } = new_http_req
            .extensions()
            .get::<TokenData<AuthToken>>()
            .unwrap()
            .claims;
        let id = id as i32;

        let user = match user_service.get_self(id).await {
            Ok(user) => user,
            Err(GetSelfError::UserNotFound) => {
                return Ok(ServiceResponse::new(new_http_req, not_found_res))
            }
            Err(GetSelfError::InternalServerError) => {
                return Ok(ServiceResponse::new(new_http_req, internal_server_err_res));
            }
        };

        if user.username == super_username && user.email == super_email {
            res_fut.await.map(ServiceResponse::map_into_left_body)
        } else {
            Ok(ServiceResponse::new(new_http_req, forbidden_res))
        }
    })
}
