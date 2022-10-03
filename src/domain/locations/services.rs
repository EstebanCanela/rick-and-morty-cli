use crate::domain::locations::types::*;

const BASE_URL: &str = "https://rickandmortyapi.com/api/location";

pub async fn get_all_locations(
    page: Option<String>,
    name: Option<String>,
    dimension: Option<String>,
) -> Result<GetAllLocationsResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut query_params = vec![];

    if page.is_some() {
        query_params.push(("page", page.unwrap()));
    };

    if name.is_some() {
        query_params.push(("name", name.unwrap()));
    };

    if dimension.is_some() {
        query_params.push(("dimension", dimension.unwrap()));
    };

    let response = client
        .get(BASE_URL.to_string())
        .query(&query_params)
        .send()
        .await?
        .json::<GetAllLocationsResponse>()
        .await?;

    return Ok(response);
}

pub async fn get_locations(ids: &Vec<String>) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/{}", BASE_URL.to_string(), ids.join(",")))
        .send()
        .await?
        .json::<Vec<Location>>()
        .await?;

    return Ok(response);
}

pub async fn get_location(id: &String) -> Result<Location, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/{}", BASE_URL.to_string(), id))
        .send()
        .await?
        .json::<Location>()
        .await?;

    return Ok(response);
}
