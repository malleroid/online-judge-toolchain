mod api;
mod commands;

use clap::{Parser, Subcommand};
use env_logger;
use log::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Login to the platform
    Login {
        #[arg(short, long)]
        service: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        password: Option<String>,
    },
    // Download problem data
    Download,
    // Submit the solution
    Submit,
    // Run test cases
    Test,
    // Generate input files
    GenerateInput,
    // Generate output files
    GenerateOutput,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();

    if cli.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        Commands::Login {
            service,
            username,
            password,
        } => {
            log::info!("Running the login command...");
            if let Err(e) =
                commands::login::login_command(&service, &username, password.as_deref()).await
            {
                log::error!("Error: {}", e);
            }
        }
        Commands::Download => {
            log::info!("Running the download command...");
            // Call the download module
        }
        Commands::Submit => {
            log::info!("Running the submit command...");
            // Call the submit module
        }
        Commands::Test => {
            log::info!("Running the test command...");
            // Call the test module
        }
        Commands::GenerateInput => {
            log::info!("Generating input files...");
            // Call the generate_input module
        }
        Commands::GenerateOutput => {
            log::info!("Generating output files...");
            // Call the generate_output module
        }
    }
}
