use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub count: u64,
    pub pages: u64,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: u64,
    pub name: String,
    pub dimension: String,
    pub residents: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllLocationsResponse {
    pub info: Info,
    pub results: Vec<Location>,
}
