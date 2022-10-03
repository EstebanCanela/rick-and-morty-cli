use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub count: u64,
    pub pages: u64,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: u64,
    pub name: String,
    pub air_date: String,
    pub episode: String,
    pub characters: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllEpisodesResponse {
    pub info: Info,
    pub results: Vec<Episode>,
}
