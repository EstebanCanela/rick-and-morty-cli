use actix_web::web::{Json, Path};
use actix_web::{get, App, HttpServer};

use crate::domain::characters::services as characters_service;
use crate::domain::characters::types::GetAllCharactersResponse;
use crate::domain::episodes::services as episodes_service;
use crate::domain::episodes::types::GetAllEpisodesResponse;
use crate::domain::locations::services as locations_service;
use crate::domain::locations::types::GetAllLocationsResponse;
use crate::proxy::types::*;

const LOCALHOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[get("/api/character")]
async fn get_all_characters() -> Json<GetAllCharactersResponse> {
    let characters = characters_service::get_all_characters(None, None, None)
        .await
        .unwrap();
    return Json(characters);
}

#[get("/api/character/{character_id}")]
async fn get_character(character_id: Path<String>) -> Json<ResponseGetCharacters> {
    return if character_id.clone().len() == 1 {
        Json(ResponseGetCharacters::SingleCharacter(
            characters_service::get_character(&character_id.clone())
                .await
                .unwrap(),
        ))
    } else {
        Json(ResponseGetCharacters::MultipleCharacters(
            characters_service::get_characters(
                &character_id
                    .clone()
                    .split(",")
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
            .await
            .unwrap(),
        ))
    };
}

#[get("/api/episode")]
async fn get_all_episodes() -> Json<GetAllEpisodesResponse> {
    let episodes_response = episodes_service::get_all_episodes(None, None, None)
        .await
        .unwrap();
    return Json(episodes_response);
}

#[get("/api/episode/{episode_id}")]
async fn get_episode(episode_id: Path<String>) -> Json<ResponseGetEpisodes> {
    return if episode_id.clone().len() == 1 {
        Json(ResponseGetEpisodes::SingleEpisode(
            episodes_service::get_episode(&episode_id.clone())
                .await
                .unwrap(),
        ))
    } else {
        Json(ResponseGetEpisodes::MultipleEpisode(
            episodes_service::get_episodes(
                &episode_id
                    .clone()
                    .split(",")
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
            .await
            .unwrap(),
        ))
    };
}

#[get("/api/location")]
async fn get_all_locations() -> Json<GetAllLocationsResponse> {
    let locations_response = locations_service::get_all_locations(None, None, None)
        .await
        .unwrap();
    return Json(locations_response);
}

#[get("/api/location/{location_id}")]
async fn get_location(location_id: Path<String>) -> Json<ResponseGetLocations> {
    return if location_id.clone().len() == 1 {
        Json(ResponseGetLocations::SingleLocation(
            locations_service::get_location(&location_id.clone())
                .await
                .unwrap(),
        ))
    } else {
        Json(ResponseGetLocations::MultipleLocation(
            locations_service::get_locations(
                &location_id
                    .clone()
                    .split(",")
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
            .await
            .unwrap(),
        ))
    };
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server listening on {}:{}...", LOCALHOST, PORT);

    HttpServer::new(|| {
        App::new()
            .service(get_all_characters)
            .service(get_character)
            .service(get_all_episodes)
            .service(get_episode)
            .service(get_all_locations)
            .service(get_location)
    })
    .bind((LOCALHOST, PORT))?
    .run()
    .await?;

    return Ok(());
}
