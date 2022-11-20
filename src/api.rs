use std::collections::HashMap;

use ureq::serde_json::Value;

pub fn list_currencies() -> Result<HashMap<String, String>, String> {
    ureq::get("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies.json")
        .call()
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())
}

pub fn list_currencies_string() -> Result<String, String> {
    Ok(list_currencies()?
        .into_iter()
        .map(|(code, name)| format!("{}: {}", code, name))
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn search_currency(query: &str) -> Result<String, String> {
    Ok(list_currencies_string()?
        .lines()
        .map(String::from)
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<_>>()
        .join("\n"))
}

pub fn convert_currency(val: f64, from: &str, to: &str) -> Result<f64, String> {
    Ok(val
        * ureq::get(&format!(
            "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{}/{}.json",
            from, to
        ))
        .call()
        .map_err(|e| e.to_string())?
        .into_json::<HashMap<String, Value>>()
        .map_err(|e| e.to_string())?[to]
            .as_f64()
            .ok_or(String::from("Not a number value"))?)
}
