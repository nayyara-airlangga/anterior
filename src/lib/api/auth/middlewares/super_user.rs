use std::{env, pin::Pin};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use jsonwebtoken::TokenData;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{jwt::payload::AuthToken, models::user::User};

type DbPool = Pool<Postgres>;

pub struct SuperUser;

impl<S, B> Transform<S, ServiceRequest> for SuperUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SuperUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SuperUserMiddleware { service })
    }
}

pub struct SuperUserMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SuperUserMiddleware<S>
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
        let forbidden_res = HttpResponse::Forbidden()
            .json(json!({
                "message": "Not permitted to access resource"
            }))
            .map_into_right_body();

        let not_found_res = HttpResponse::NotFound()
            .json(json!({
                "message": "User not found"
            }))
            .map_into_right_body();

        let internal_server_err_res = HttpResponse::InternalServerError()
            .finish()
            .map_into_right_body();

        let (http_req, payload) = req.into_parts();
        let new_http_req = http_req.clone();
        let new_req = ServiceRequest::from_parts(http_req, payload);
        let res_fut = self.service.call(new_req);

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

            let pool = new_http_req.app_data::<web::Data<DbPool>>().unwrap();

            let AuthToken { id, .. } = new_http_req
                .extensions()
                .get::<TokenData<AuthToken>>()
                .unwrap()
                .claims;
            let id = id as i32;

            let user = match sqlx::query_as::<Postgres, User>(
                "
SELECT id, username, name, email, created_at FROM posterior.users
WHERE id = $1
",
            )
            .bind(&id)
            .fetch_optional(&***pool)
            .await
            {
                Ok(query) => query,
                Err(err) => {
                    log::error!("{err}");
                    return Ok(ServiceResponse::new(new_http_req, internal_server_err_res));
                }
            };

            if let Some(user) = user {
                if user.username == super_username && user.email == super_email {
                    res_fut.await.map(ServiceResponse::map_into_left_body)
                } else {
                    Ok(ServiceResponse::new(new_http_req, forbidden_res))
                }
            } else {
                Ok(ServiceResponse::new(new_http_req, not_found_res))
            }
        })
    }
}
