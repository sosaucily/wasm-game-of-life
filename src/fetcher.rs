use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::log;

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
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // serde_json::from_str(&resp_value.text()).unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let branch_info: Branch = serde_wasm_bindgen::from_value::<Branch>(json)?;

    // let branch_info: Branch = serde_json::from_str(&json).unwrap();

    log!("Branch info: {:#?}", branch_info);
    // Send the JSON response back to JS.
    Ok(serde_wasm_bindgen::to_value(&branch_info).unwrap())

    // let res = match reqwest::Client::new()
    //     .get("")
    //     .header("Accept", "application/vnd.github.v3+json")
    //     .send()
    //     .await
    // {
    //     Ok(res) => res,
    //     Err(err) => {
    //         log!("Error: {}", err);
    //         return Err(JsValue::from_str(&format!("Error: {}", err)));
    //     }
    // };

    // let text = match res.text().await {
    //     Ok(text) => text,
    //     Err(err) => {
    //         log!("Error: {}", err);
    //         return Err(JsValue::from_str(&format!("Error: {}", err)));
    //     }
    // };
    // let branch_info: Branch = serde_json::from_str(&text).unwrap();
    // log!("Branch info: {:#?}", branch_info);

    // Ok(serde_wasm_bindgen::to_value(&branch_info).unwrap())
}
