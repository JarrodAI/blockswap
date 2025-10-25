use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
	pub chain: Arc<Mutex<crate::blockchain::chain::Chain>>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest<T = serde_json::Value> {
	pub jsonrpc: String,
	pub method: String,
	pub params: Option<T>,
	pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse<R = serde_json::Value> {
	pub jsonrpc: &'static str,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<R>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<JsonRpcError>,
	pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
	pub code: i64,
	pub message: String,
}

pub async fn start_http_server(port: u16, chain: Arc<Mutex<crate::blockchain::chain::Chain>>) -> Result<()> {
	let state = AppState { chain };
	let app = Router::new()
		.route("/health", get(health))
		.route("/rpc", post(rpc_handler))
		.with_state(state);

	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	let listener = TcpListener::bind(&addr).await?;
	info!(%addr, "http rpc listening");
	axum::serve(listener, app).await?;
	Ok(())
}

async fn health() -> impl IntoResponse {
	(StatusCode::OK, "ok")
}

async fn rpc_handler(
	State(state): State<AppState>,
	Json(req): Json<JsonRpcRequest>,
) -> impl IntoResponse {
	let id = req.id.clone();
	match req.method.as_str() {
		"chain_getHeight" => {
			let h = {
				let chain = state.chain.lock().await;
				chain.get_height()
			};
			Json(JsonRpcResponse {
				jsonrpc: "2.0",
				result: Some(serde_json::json!(h)),
				error: None,
				id,
			})
		}
		_ => Json(JsonRpcResponse::<serde_json::Value> {
			jsonrpc: "2.0",
			result: None,
			error: Some(JsonRpcError { code: -32601, message: "Method not found".into() }),
			id,
		}),
	}
}
