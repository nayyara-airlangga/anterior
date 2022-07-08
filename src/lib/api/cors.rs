use actix_cors::Cors;

pub fn config_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .supports_credentials()
        .max_age(3600)
}
