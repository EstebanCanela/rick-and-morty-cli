pub mod args;

use clap::Parser;

use crate::domain::characters::services as characters_service;
use crate::domain::episodes::services as episodes_service;
use crate::domain::locations::services as locations_service;

use crate::proxy::server as proxy_server;

use crate::cli::args::{
    CharactersSubcommand, EntityType, EpisodesSubcommand, LocationsSubcommand, ProxySubCommand,
    RickAndMortyArgs,
};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = RickAndMortyArgs::parse();

    match args.entity_type {
        EntityType::Characters(characters_command) => {
            match characters_command.command {
                CharactersSubcommand::GetAll(params) => {
                    println!(
                        "{:?}",
                        characters_service::get_all_characters(
                            params.page,
                            params.status,
                            params.gender
                        )
                        .await?
                    );
                }
                CharactersSubcommand::Get(params) => {
                    println!("{:?}", characters_service::get_character(&params.id).await?);
                }
                CharactersSubcommand::GetMultiple(params) => {
                    println!(
                        "{:?}",
                        characters_service::get_characters(&params.ids).await?
                    );
                }
            };
        }
        EntityType::Episodes(episodes_command) => {
            match episodes_command.command {
                EpisodesSubcommand::GetAll(params) => {
                    println!(
                        "{:?}",
                        episodes_service::get_all_episodes(
                            params.page,
                            params.name,
                            params.episode
                        )
                        .await?
                    );
                }
                EpisodesSubcommand::Get(params) => {
                    println!("{:?}", episodes_service::get_episode(&params.id).await?);
                }
                EpisodesSubcommand::GetMultiple(params) => {
                    println!("{:?}", episodes_service::get_episodes(&params.ids).await?);
                }
            };
        }
        EntityType::Locations(locations_command) => {
            match locations_command.command {
                LocationsSubcommand::GetAll(params) => {
                    println!(
                        "{:?}",
                        locations_service::get_all_locations(
                            params.page,
                            params.name,
                            params.dimension
                        )
                        .await?
                    );
                }
                LocationsSubcommand::Get(params) => {
                    println!("{:?}", locations_service::get_location(&params.id).await?);
                }
                LocationsSubcommand::GetMultiple(params) => {
                    println!("{:?}", locations_service::get_locations(&params.ids).await?);
                }
            };
        }
        EntityType::Proxy(proxy_command) => match proxy_command.command {
            ProxySubCommand::SpinUp => {
                proxy_server::run().await?;
            }
        },
    };

    return Ok(());
}
