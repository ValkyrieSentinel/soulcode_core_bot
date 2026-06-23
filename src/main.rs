mod encoder;
mod locales;

use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::post,
    Json, Router,
};
use dotenvy::dotenv;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::cors::CorsLayer;
use uuid::Uuid;


extern "C" {
    fn validate_p2p_signature(amount: f64, order_id_ptr: *const u8, id_len: usize) -> bool;
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}


#[derive(Deserialize)]
struct EncodeInput {
    user_id: u64,
    addresses: Vec<String>,
    amounts: Vec<String>,
}

#[derive(Serialize)]
struct EncodeOutput {
    success: bool,
    calldata: Option<String>,
    error: Option<String>,
}


#[derive(Deserialize)]
struct P2pCreateInput {
    user_id: u64,
    uah_amount: f64,
    recipients_json: String, 
}

#[derive(Serialize)]
struct P2pCreateOutput {
    success: bool,
    order_id: Option<String>,
    card_number: Option<String>,
    error: Option<String>,
}

async fn api_encode_handler(
    State(state): State<AppState>,
    Json(payload): Json<EncodeInput>,
) -> (StatusCode, Json<EncodeOutput>) {
    let is_pro = {
        let conn = state.db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT 1 FROM premium_users WHERE user_id = ?1").unwrap();
        stmt.exists([payload.user_id]).unwrap_or(false)
    };

    if !is_pro {
        return (StatusCode::FORBIDDEN, Json(EncodeOutput {
            success: false, calldata: None, error: Some("No active subscription".to_string())
        }));
    }

    match encoder::AirdropEncoder::encode_airdrop(payload.addresses, payload.amounts) {
        Ok(cd) => (StatusCode::OK, Json(EncodeOutput { success: true, calldata: Some(cd), error: None })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(EncodeOutput { success: false, calldata: None, error: Some(e) })),
    }
}

async fn api_p2p_create_handler(
    State(state): State<AppState>,
    Json(payload): Json<P2pCreateInput>,
) -> (StatusCode, Json<P2pCreateOutput>) {
    let order_id = Uuid::new_v4().to_string();

    
    let is_safe = unsafe {
        validate_p2p_signature(payload.uah_amount, order_id.as_ptr(), order_id.len())
    };

    if !is_safe {
        return (StatusCode::BAD_REQUEST, Json(P2pCreateOutput {
            success: false, order_id: None, card_number: None, error: Some("Invalid order signature detected by Zig".to_string())
        }));
    }

    
    let mock_trader_card = "4441111122223333".to_string(); 

    
    {
        let conn = state.db.lock().unwrap();
        let res = conn.execute(
            "INSERT INTO p2p_orders (order_id, user_id, uah_amount, status) VALUES (?1, ?2, ?3, 'Pending')",
            (&order_id, &payload.user_id, &payload.uah_amount),
        );
        if res.is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(P2pCreateOutput {
                success: false, order_id: None, card_number: None, error: Some("Database error".to_string())
            }));
        }
    }

    (StatusCode::OK, Json(P2pCreateOutput {
        success: true,
        order_id: Some(order_id),
        card_number: Some(mock_trader_card),
        error: None,
    }))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "subs.db".to_string());
    let conn = Connection::open(db_path).expect("Failed to open DB");
    
    
    conn.execute("CREATE TABLE IF NOT EXISTS premium_users (user_id INTEGER PRIMARY KEY)", []).unwrap();
    
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS p2p_orders (
            order_id TEXT PRIMARY KEY, 
            user_id INTEGER, 
            uah_amount REAL, 
            status TEXT
        )", 
        []
    ).unwrap();
    
    let state = AppState { db: Arc::new(Mutex::new(conn)) };

    
    let app = Router::new()
        .route("/api/encode", post(api_encode_handler))
        .route("/api/p2p/create", post(api_p2p_create_handler)) 
        .layer(
            CorsLayer::new()
                .allow_origin("https://app.soulcode.studio".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
        .with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    
    tracing::info!("SoulCode API + P2P Gateway started on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}