use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use dotenv::dotenv;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, time::{SystemTime, UNIX_EPOCH}};
use gl_client::{
    bitcoin::Network,
    credentials::{Nobody, Device},
    node::ClnClient,
    pb::cln::OfferRequest,
    scheduler::Scheduler,
    signer::Signer,
};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct CreateOfferParams {
    username: String,
    expiry: Option<u32>,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
    offer: Option<String>,
    dns_status: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new().route("/api/create-offer", get(create_offer));

    let addr: SocketAddr = ([127, 0, 0, 1], 8081).into();
    println!("✅ Backend running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_offer(Query(params): Query<CreateOfferParams>) -> (StatusCode, Json<ApiResponse>) {
    let username = params.username;
    let expiry = params.expiry;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let cert_path = env::var("GL_CERT_PATH").unwrap_or_else(|_| "certs/client.crt".to_string());
    let key_path = env::var("GL_KEY_PATH").unwrap_or_else(|_| "certs/client-key-pkcs8.pem".to_string());

    let developer_creds = Nobody {
        cert: std::fs::read(cert_path.clone()).unwrap(),
        key: std::fs::read(key_path.clone()).unwrap(),
        ..Default::default()
    };

    let network = Network::Bitcoin;

    let scheduler = Scheduler::new(network, developer_creds.clone()).await.unwrap();
    let mut seed = [0u8; 32];
    thread_rng().fill_bytes(&mut seed);
    let signer = Signer::new(seed.to_vec(), network, developer_creds).unwrap();
    let registration = scheduler.register(&signer, None).await.unwrap();
    let device_creds = Device::from_bytes(registration.creds);
    let scheduler = scheduler.authenticate(device_creds).await.unwrap();
    let mut node: ClnClient = scheduler.node().await.unwrap();

    let offer_request = OfferRequest {
        amount: "any".to_string(),
        description: Some(format!("Shopstr username '{}'", username)),
        issuer: Some("Shopstr".to_string()),
        label: Some(username.clone()),
        absolute_expiry: expiry.map(|e| timestamp + e as u64),
        ..Default::default()
    };

    match node.offer(offer_request).await {
        Ok(offer) => {
            let bolt12 = offer.get_ref().bolt12.clone();

            // DNS Record update (simplified)
            let dns_status = update_dns(&username, &bolt12).await;

            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    message: "Offer created successfully".to_string(),
                    offer: Some(bolt12),
                    dns_status: Some(dns_status),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to create offer: {}", e),
                offer: None,
                dns_status: None,
            }),
        ),
    }
}

async fn update_dns(username: &str, offer: &str) -> String {
    let cf_api = env::var("CLOUDFLARE_API").unwrap();
    let zone_id = env::var("CLOUDFLARE_ZONE_ID").unwrap();
    let _domain = env::var("DOMAIN").unwrap(); // Optional use

    let client = reqwest::Client::new();
    let dns_name = format!("_bitcoin._tcp.{}", username);
    let record_value = format!("offer={}", offer);

    let body = serde_json::json!({
        "type": "TXT",
        "name": dns_name,
        "content": record_value,
        "ttl": 3600,
    });

    let res = client
        .post(&format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            zone_id
        ))
        .header("Authorization", format!("Bearer {}", cf_api))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match res {
        Ok(r) => {
            if r.status().is_success() {
                "DNS record created successfully".to_string()
            } else {
                format!("Failed to create DNS record: {}", r.status())
            }
        }
        Err(e) => format!("DNS error: {}", e),
    }
}