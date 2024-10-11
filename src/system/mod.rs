pub mod ping;
use axum::Router;

pub fn routes() -> Router {
    Router::new().typed_get(ping::ping)
}
