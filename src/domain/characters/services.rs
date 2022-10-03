use crate::domain::characters::types::*;

const BASE_URL: &str = "https://rickandmortyapi.com/api/character";

pub async fn get_all_characters(
    page: Option<String>,
    status: Option<String>,
    gender: Option<String>,
) -> Result<GetAllCharactersResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut query_params = vec![];

    if page.is_some() {
        query_params.push(("page", page.unwrap()));
    };

    if status.is_some() {
        query_params.push(("status", status.unwrap()));
    };

    if gender.is_some() {
        query_params.push(("gender", gender.unwrap()));
    };

    let response = client
        .get(BASE_URL.to_string())
        .query(&query_params)
        .send()
        .await?
        .json::<GetAllCharactersResponse>()
        .await?;

    return Ok(response);
}

pub async fn get_characters(
    ids: &Vec<String>,
) -> Result<Vec<Character>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let ids_query_param: String;
    if ids.len() > 1 {
        ids_query_param = ids.join(",");
    } else {
        ids_query_param = ids[0].clone();
    }

    let response = client
        .get(format!("{}/{}", BASE_URL.to_string(), ids_query_param))
        .send()
        .await?
        .json::<Vec<Character>>()
        .await?;

    return Ok(response);
}

pub async fn get_character(id: &String) -> Result<Character, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/{}", BASE_URL.to_string(), id))
        .send()
        .await?
        .json::<Character>()
        .await?;

    return Ok(response);
}
