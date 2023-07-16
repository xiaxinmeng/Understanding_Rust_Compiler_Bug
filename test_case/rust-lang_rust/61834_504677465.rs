rust
#![feature(async_await)]

use futures::compat::Future01CompatExt;
use reqwest::r#async::Client;
use serde_json::Value;

type Map = serde_json::Map<String, Value>;

async fn get<'a, D, Q>(client: &'a Client, path: &'a str, query: Q) -> Result<D, String>
where
    Q: serde::Serialize,
    D: serde::de::DeserializeOwned,
{
    let value = serde_json::to_value(query).map_err(|e| e.to_string())?;

    let query_map = match value {
        serde_json::Value::Object(obj) => obj,
        _ => Err(format!(
            "Invalid query data: type must serialize into a object"
        ))?,
    };
    let mut clean_map: Map = query_map
        .into_iter()
        .filter(|(_key, value)| !value.is_null())
        .collect();
    clean_map.insert("key".into(), "value".into());

    let url = "".to_string();
    let mut res = client
        .get(&url)
        .query(&clean_map)
        .send()
        .compat()
        .await
        .map_err(|e| e.to_string());

    Err("err".to_string())
}

