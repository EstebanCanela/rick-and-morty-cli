use crate::domain::characters::types::Character;
use crate::domain::episodes::types::Episode;
use crate::domain::locations::types::Location;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseGetCharacters {
    SingleCharacter(Character),
    MultipleCharacters(Vec<Character>),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseGetLocations {
    SingleLocation(Location),
    MultipleLocation(Vec<Location>),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseGetEpisodes {
    SingleEpisode(Episode),
    MultipleEpisode(Vec<Episode>),
}
