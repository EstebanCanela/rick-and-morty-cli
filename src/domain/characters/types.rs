use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Origin {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub species: String,
    pub gender: String,
    pub origin: Origin,
    pub image: String,
    pub episode: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub count: u64,
    pub pages: u64,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllCharactersResponse {
    pub info: Info,
    pub results: Vec<Character>,
}
