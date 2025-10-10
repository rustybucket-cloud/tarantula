use crate::app::install;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tarantula", version, about = "Use web apps like desktop apps")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run(RunArgs),
    Install(InstallArgs),
    Uninstall,
    List,
    Update,
}

#[derive(Debug, Args)]
struct RunArgs {
    name: String,
}

#[derive(Debug, Args)]
struct InstallArgs {
    name: String,
    url: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => {
            println!("Run app: {}", args.name);
        }
        Commands::Install(args) => match install::install(args.name.as_str(), args.url.as_str()) {
            Ok(_) => println!("Web app installed!"),
            Err(install::InstallError::Io(e)) => {
                eprint!("Error installing app: {}", e);
            }
        },
        Commands::Uninstall => {
            println!("Uninstall");
        }
        Commands::List => {
            println!("List");
        }
        Commands::Update => {
            println!("Update");
        }
    }

    Ok(())
}
