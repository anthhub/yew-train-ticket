#![allow(unused_variables)]
use super::fetch::Fetch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCityResult {
    pub result: Vec<SearchResult>,
    pub searchKey: String,
}

impl SearchCityResult {
    pub fn new() -> SearchCityResult {
        SearchCityResult {
            result: vec![],
            searchKey: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub key: String,
    pub display: String,
}

pub async fn search_city_list(search_word: String) -> SearchCityResult {
    // let base_url = "http://118.190.37.169:7000";
    let base_url = "http://att.liuma.top";
    let url = format!("{}/api/search?key={}", base_url, search_word);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => json.into_serde().unwrap(),
        Err(_) => SearchCityResult::new(),
    }
}
