#![allow(unused_variables)]
use super::fetch::Fetch;
use serde::{Deserialize, Serialize,};


#[derive(Debug, Serialize, Deserialize)]
pub struct CityResult {
    pub hotCities: Vec<HotCities>,
    pub cityList: Vec<CityList>,
    pub version: u32,
}

impl CityResult {
  pub  fn new() -> CityResult{
    CityResult{hotCities : vec![] , cityList : vec![] ,version :0 }
   }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotCities {
    pub  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityList {
    pub title: String,

    #[serde(default)]
    pub citys: Citys,
}

type Citys = Vec<City>;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub name: String,
}


pub async fn get_city_list() -> CityResult {
    let base_url = "http://118.190.37.169:7000";
    let url = format!("{}/api/cities", base_url);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => json.into_serde().unwrap(),
        Err(_) => CityResult::new(),
    }
   
}
