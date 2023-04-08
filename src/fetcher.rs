use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::log;

// use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[wasm_bindgen]
pub async fn run_git_fetch_test() -> Result<JsValue, JsValue> {
    log!("Fetching branch info...");
    let res = match reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            log!("Error: {}", err);
            return Err(JsValue::from_str(&format!("Error: {}", err)));
        }
    };

    let text = match res.text().await {
        Ok(text) => text,
        Err(err) => {
            log!("Error: {}", err);
            return Err(JsValue::from_str(&format!("Error: {}", err)));
        }
    };
    let branch_info: Branch = serde_json::from_str(&text).unwrap();
    log!("Branch info: {:#?}", branch_info);

    Ok(serde_wasm_bindgen::to_value(&branch_info).unwrap())
}
