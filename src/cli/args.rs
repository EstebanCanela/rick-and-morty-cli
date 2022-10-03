use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RickAndMortyArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Get all or a single character
    Characters(CharactersCommand),

    /// Get all or a single episode
    Episodes(EpisodesCommand),

    /// Get all or a single location
    Locations(LocationsCommand),

    /// Spin up proxy server
    Proxy(ProxyCommand),
}

#[derive(Debug, Args)]
pub struct ProxyCommand {
    #[clap(subcommand)]
    pub command: ProxySubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ProxySubCommand {
    /// Spin up proxy web server
    SpinUp,
}

#[derive(Debug, Args)]
pub struct CharactersCommand {
    #[clap(subcommand)]
    pub command: CharactersSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CharactersSubcommand {
    /// Get all
    GetAll(GetAllCharactersParams),

    /// Get by id
    Get(GetByIdParams),

    /// Get multiple by ids
    GetMultiple(GetByIdsParams),
}

#[derive(Debug, Args)]
pub struct GetAllLocationsParams {
    /// Optional: Page number
    #[arg(short, long)]
    pub page: Option<String>,

    /// Optional: Filter by the given name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Optional: Filter by the given dimension
    #[arg(short, long)]
    pub dimension: Option<String>,
}

#[derive(Debug, Args)]
pub struct GetAllEpisodesParams {
    /// Optional: Page number
    #[arg(short, long)]
    pub page: Option<String>,

    /// Optional: Filter by the given name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Optional: Filter by the given episode code
    #[arg(short, long)]
    pub episode: Option<String>,
}

#[derive(Debug, Args)]
pub struct GetAllCharactersParams {
    /// Optional: Page number
    #[arg(short, long)]
    pub page: Option<String>,

    /// Optional: Filter by the given status - Values (alive, dead or unknown)
    #[arg(short, long)]
    pub status: Option<String>,

    /// Optional: Filter by the given gender  - Values (female, male, genderless or unknown)
    #[arg(short, long)]
    pub gender: Option<String>,
}

#[derive(Debug, Args)]
pub struct GetByIdParams {
    /// ID
    #[arg(short, long)]
    pub id: String,
}

#[derive(Debug, Args)]
pub struct GetByIdsParams {
    /// ID - Example 1,2,3
    #[arg(short, long)]
    pub ids: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EpisodesCommand {
    #[clap(subcommand)]
    pub command: EpisodesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EpisodesSubcommand {
    /// Get all
    GetAll(GetAllEpisodesParams),

    /// Get by id
    Get(GetByIdParams),

    /// Get multiple by ids
    GetMultiple(GetByIdsParams),
}

#[derive(Debug, Args)]
pub struct LocationsCommand {
    #[clap(subcommand)]
    pub command: LocationsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum LocationsSubcommand {
    /// Get all
    GetAll(GetAllLocationsParams),

    /// Get by id
    Get(GetByIdParams),

    /// Get multiple by ids
    GetMultiple(GetByIdsParams),
}
