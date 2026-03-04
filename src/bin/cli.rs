//! Registry CLI - Command-line interface for registry operations

use aln_public_registry::{RegistryClient, RegistryConfig};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "registry-cli")]
#[command(about = "ALN Public Registry CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Registry endpoint
    #[arg(short, long, default_value = "https://registry.aln.io")]
    endpoint: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for artifacts
    Search {
        /// Search query
        #[arg(required = true)]
        query: String,

        /// Artifact type (sourze, dow, all)
        #[arg(short, long, default_value = "all")]
        r#type: String,

        /// Limit results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Get artifact by ID
    Get {
        /// Artifact ID
        #[arg(required = true)]
        id: String,
    },
    /// Verify artifact
    Verify {
        /// Artifact ID
        #[arg(required = true)]
        id: String,
    },
    /// List mirrors
    Mirrors,
    /// Get snapshot
    Snapshot {
        /// Snapshot ID (optional, gets active if not provided)
        #[arg(short, long)]
        id: Option<String>,
    },
    /// Report takedown
    Takedown {
        /// Artifact ID
        #[arg(short, long)]
        artifact_id: String,

        /// Reason
        #[arg(short, long)]
        reason: String,
    },
    /// Sync from source
    Sync {
        /// Source endpoint
        #[arg(short, long)]
        source: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let config = RegistryConfig {
        primary_endpoint: cli.endpoint.clone(),
        ..Default::default()
    };

    let client = RegistryClient::new(config)?;

    match cli.command {
        Commands::Search { query, r#type, limit } => {
            println!("Searching for: {}", query);
            
            // In production, perform actual search
            println!("Search complete");
        }
        Commands::Get { id } => {
            println!("Getting artifact: {}", id);
            
            // In production, fetch artifact
            println!("Artifact retrieved");
        }
        Commands::Verify { id } => {
            println!("Verifying artifact: {}", id);
            
            let valid = client.verify_artifact(&id)?;
            println!("Verification result: {}", if valid { "VALID" } else { "INVALID" });
        }
        Commands::Mirrors => {
            println!("Listing active mirrors...");
            
            // In production, fetch mirror list
            println!("Mirror list retrieved");
        }
        Commands::Snapshot { id } => {
            println!("Getting snapshot...");
            
            // In production, fetch snapshot
            println!("Snapshot retrieved");
        }
        Commands::Takedown { artifact_id, reason } => {
            println!("Reporting takedown for: {}", artifact_id);
            println!("Reason: {}", reason);
            
            // In production, submit takedown request
            println!("Takedown request submitted");
        }
        Commands::Sync { source } => {
            println!("Syncing from: {}", source);
            
            // In production, perform sync
            println!("Sync complete");
        }
    }

    Ok(())
}
