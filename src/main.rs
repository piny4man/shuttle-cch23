use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

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

    if !numbers.is_empty() && numbers.len() <= 20 {
        let result: u64 = numbers.iter().fold(0, |acc, &num| acc ^ num).pow(3);
        result.to_string()
    } else {
        // Handle invalid number of packets
        "Invalid number of packets (1-20)".to_string()
    }
}

async fn calculate_reeindeers_strength(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    let total_strength: i32 = reindeers.iter().map(|r| r.strength).sum();
    total_strength.to_string()
}

async fn reindeers_contest_results() -> &'static str {
    "The winner is YOU"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_handler))
        .route("/1/*num_list", get(packed_recalibration))
        .route("/4/strength", post(calculate_reeindeers_strength))
        .route("/4/contest", post(reindeers_contest_results));

    Ok(router.into())
}
