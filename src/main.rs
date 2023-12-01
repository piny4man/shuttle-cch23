use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_handler() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn packed_recalibration(Path((num1, num2)): Path<(u64, u64)>) -> String {
    (num1 ^ num2).pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_handler))
        .route("/1/:num1/:num2", get(packed_recalibration));

    Ok(router.into())
}
