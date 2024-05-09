use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::fs;
use serde::Deserialize;
use toml;

#[derive(Parser)]
#[command(author = "Kubik Labs")]
#[command(version = "1.0")]
#[command(about = "Inges is an indexer for Cosmos based Blockchain")]
#[command(long_about = "Inges is an indexer for Cosmos based Blockchain developed by Kubik Labs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Runs the Inges indexer as per the specified config file
    Run(RunArgs),

    /// Initializes the Inges indexer's database as per the specified schema
    InitDb(InitDbArgs),

    /// Validates the Inges indexer's database by running some tests
    ValidateDb(ValidateDbArgs),
}

#[derive(Args)]
struct RunArgs {
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Args)]
struct InitDbArgs {
    #[arg(short, long)]
    schema: Option<PathBuf>,
}

#[derive(Args)]
struct ValidateDbArgs {
    #[arg(short, long)]
    db: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
struct ConfigToml{
    websocket_url : String,
    rpc_url: String,
}

fn main() {
    let cli = Cli::parse();
    let mut config_toml: ConfigToml = ConfigToml{
        websocket_url: "".to_owned(),
        rpc_url: "".to_owned()
};
    match &cli.command {

        Commands::Run(_config) => {
            if let Some(config_path) = &_config.config {
                if let Ok(content) = fs::read_to_string(config_path) {
                    match toml::from_str::<ConfigToml>(&content) {
                        Ok(parsed_config) => {
                            // Validation logic goes here if needed
                            config_toml = parsed_config;
                            println!("'inges run' was used, parsed and validated config: {:#?}", config_toml);
                        }
                        Err(err) => {
                            eprintln!("Error parsing the config file as TOML: {:?}", err);
                        }
                    }
                } else {
                    eprintln!("Error reading the config file: {:?}", config_path);
                }
            }
        }

        Commands::InitDb(_schema) => {
            println!("'inges init-db' was used, name is: {:?}", _schema.schema);
        }

        Commands::ValidateDb(_db) => {
            println!("'inges validate-db' was used, name is: {:?}", _db.db);
        }
    }
}