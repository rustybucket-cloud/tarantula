use dirs;

use crate::app::install;
use crate::app::run;
use crate::app::uninstall;
use crate::app::update;
use crate::infra::app_data;
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
    Uninstall(UpdateArgs),
    List,
    Update(UpdateArgs),
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

#[derive(Debug, Args)]
struct UninstallArgs {
    name: String,
}

#[derive(Debug, Args)]
struct UpdateArgs {
    name: String,

    #[arg(short = 'n', long = "name")]
    new_name: Option<String>,

    #[arg(short = 'u', long = "url")]
    new_url: Option<String>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let app_data_path = home_dir
        .join(".local/share/tarantula")
        .join("apps.json")
        .to_path_buf();
    let desktop_data_path = home_dir.join(".local/share/applications").to_path_buf();
    let config = crate::app::config::Config::new(app_data_path, desktop_data_path);

    match &cli.command {
        Commands::Run(args) => match run::run(args.name.as_str(), &config) {
            Ok(_) => println!("App launched!"),
            Err(run::RunError::AppNotFound(name)) => {
                eprint!("App not found: {}", name);
            }
            Err(run::RunError::LaunchFailed(reason)) => {
                eprint!("Failed to launch app: {}", reason);
            }
            Err(run::RunError::Io(e)) => {
                eprint!("Error launching app: {}", e);
            }
        },
        Commands::Install(args) => {
            match install::install(args.name.as_str(), args.url.as_str(), &config) {
                Ok(_) => println!("Web app installed!"),
                Err(install::InstallError::Io(e)) => {
                    eprint!("Error installing app: {}", e);
                }
                Err(e) => {
                    eprint!("Error installing app: {:?}", e);
                }
            }
        }
        Commands::Uninstall(args) => match uninstall::uninstall(args.name.as_str(), &config) {
            Ok(_) => println!("Web app uninstalled!"),
            Err(uninstall::UninstallError::AppNotFound) => {
                eprint!("App not found: {}", args.name);
            }
            Err(uninstall::UninstallError::Io(e)) => {
                eprint!("Error uninstalling app: {}", e);
            }
        },
        Commands::List => {
            let apps = app_data::get_apps(&config).unwrap_or_else(|_| vec![]);
            println!("Installed web apps:");
            for app in apps {
                println!("{} - {}", app.name, app.url);
            }
        }
        Commands::Update(args) => {
            let options = update::UpdateOptions {
                name: args.new_url.clone(),
                new_name: args.new_name.clone(),
            };
            match update::update(args.name.as_str(), &options, &config) {
                Ok(_) => println!("App updated!"),
                Err(update::UpdateError::AppNotFound) => {
                    eprint!("App not found: {}", args.name);
                }
                Err(update::UpdateError::Io(e)) => {
                    eprint!("Error updating app: {}", e);
                }
            }
            println!("Update");
        }
    }

    Ok(())
}
