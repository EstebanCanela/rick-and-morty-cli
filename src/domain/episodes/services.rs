use crate::domain::episodes::types::*;

const BASE_URL: &str = "https://rickandmortyapi.com/api/episode";

pub async fn get_all_episodes(
    page: Option<String>,
    name: Option<String>,
    episode: Option<String>,
) -> Result<GetAllEpisodesResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut query_params = vec![];

    if page.is_some() {
        query_params.push(("page", page.unwrap()));
    };

    if name.is_some() {
        query_params.push(("name", name.unwrap()));
    };

    if episode.is_some() {
        query_params.push(("episode", episode.unwrap()));
    };

    let body = client
        .get(BASE_URL.to_string())
        .query(&query_params)
        .send()
        .await?
        .json::<GetAllEpisodesResponse>()
        .await?;

    return Ok(body);
}

pub async fn get_episodes(ids: &Vec<String>) -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!("{}/{}", BASE_URL.to_string(), ids.join(",")))
        .send()
        .await?
        .json::<Vec<Episode>>()
        .await?;

    return Ok(body);
}

pub async fn get_episode(id: &String) -> Result<Episode, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!("{}/{}", BASE_URL.to_string(), id))
        .send()
        .await?
        .json::<Episode>()
        .await?;

    return Ok(body);
}
