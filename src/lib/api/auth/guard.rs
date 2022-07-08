use actix_web::guard::{self, Guard};

use crate::jwt::handlers::decode_auth_token;

pub struct JwtGuard;

impl Guard for JwtGuard {
    fn check(&self, ctx: &guard::GuardContext<'_>) -> bool {
        let auth_header = ctx.head().headers().get("Authorization");

        match auth_header {
            Some(header) => {
                if let Ok(header) = header.to_str() {
                    if header.starts_with("Bearer") {
                        let header_vec: Vec<&str> = header.split_whitespace().collect();
                        let token = header_vec.get(1);

                        if let Some(token) = token {
                            if let Ok(payload) = decode_auth_token(token) {
                                ctx.req_data_mut().insert(payload);
                                return true;
                            }

                            return false;
                        }
                        return false;
                    }
                }
                false
            }
            None => false,
        }
    }
}
