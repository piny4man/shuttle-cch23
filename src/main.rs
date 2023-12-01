use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

// Day -1
async fn error_handler() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

// Day 1
async fn packed_recalibration(Path(num_list): Path<String>) -> String {
    let num_strings: Vec<&str> = num_list.split('/').collect();
    let mut numbers: Vec<u64> = Vec::new();

    for num_str in num_strings {
        if let Ok(num) = num_str.parse::<u64>() {
            numbers.push(num);
        } else {
            // Handle invalid number parsing
            return "Invalid number format".to_string();
        }
    }

    if numbers.len() >= 1 && numbers.len() <= 20 {
        let result: u64 = numbers.iter().fold(0, |acc, &num| acc ^ num).pow(3);
        result.to_string()
    } else {
        // Handle invalid number of packets
        "Invalid number of packets (1-20)".to_string()
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_handler))
        .route("/1/*num_list", get(packed_recalibration));

    Ok(router.into())
}
