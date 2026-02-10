use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::constants::RPC_URL;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: u32,
    method: String,
    params: Value,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse {
    pub result: Option<Value>,
    pub error: Option<Value>,
}

pub async fn rpc_request(method: &str, params: Value) -> Result<Value, String> {
    let body = RpcRequest {
        jsonrpc: "2.0",
        id: 1,
        method: method.to_string(),
        params,
    };

    let resp = Request::post(RPC_URL)
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| format!("Request build error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let rpc_resp: RpcResponse = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;

    if let Some(err) = rpc_resp.error {
        return Err(format!("RPC error: {}", err));
    }

    rpc_resp.result.ok_or_else(|| "No result in response".to_string())
}

pub async fn get_token_accounts_by_owner(owner: &str) -> Result<Value, String> {
    let params = json!([
        owner,
        { "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" },
        { "encoding": "jsonParsed" }
    ]);
    rpc_request("getTokenAccountsByOwner", params).await
}

pub async fn get_latest_blockhash() -> Result<String, String> {
    let result = rpc_request("getLatestBlockhash", json!([])).await?;
    result["value"]["blockhash"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Missing blockhash".to_string())
}

#[allow(dead_code)]
pub async fn get_balance(pubkey: &str) -> Result<u64, String> {
    let result = rpc_request("getBalance", json!([pubkey])).await?;
    result["value"]
        .as_u64()
        .ok_or_else(|| "Invalid balance".to_string())
}
