use dirs;

use clap::{Args, Parser, Subcommand};
use shared::app::config;
use shared::app::install;
use shared::app::run;
use shared::app::uninstall;
use shared::app::update;
use shared::infra::app_data;

#[derive(Parser, Debug)]
#[command(name = "tarantula", version, about = "Use web apps like desktop apps")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    run_cmd: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install(InstallArgs),
    Uninstall(UpdateArgs),
    List,
    Update(UpdateArgs),
    Config(ConfigArgs),
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

#[derive(Debug, Args)]
struct ConfigArgs {
    #[arg(short = 'b', long = "browser")]
    browser_path: Option<String>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut config = match config::create_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error creating config: {:?}", e);
            std::process::exit(1);
        }
    };

    match &cli.command {
        Some(Commands::Install(args)) => {
            match install::install(args.name.as_str(), args.url.as_str(), &config) {
                Ok(_) => println!("Web app installed!"),
                Err(install::InstallError::InvalidData(e)) => {
                    eprint!("{}\n", e);
                }
                Err(e) => {
                    eprint!("There was a problem installing the app: {:?}", e);
                }
            }
        }
        Some(Commands::Uninstall(args)) => {
            match uninstall::uninstall(args.name.as_str(), &config) {
                Ok(_) => println!("Web app uninstalled!"),
                Err(uninstall::UninstallError::AppNotFound) => {
                    eprint!("App not found: {}", args.name);
                }
                Err(uninstall::UninstallError::Io(e)) => {
                    eprint!("Error uninstalling app: {}", e);
                }
            }
        }
        Some(Commands::List) => {
            let apps = app_data::get_apps(&config).unwrap_or_else(|_| vec![]);
            for app in apps {
                println!("{} - {}", app.name, app.url);
            }
        }
        Some(Commands::Update(args)) => {
            let options = update::UpdateOptions {
                name: args.new_name.clone(),
                url: args.new_url.clone(),
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
        }
        Some(Commands::Config(args)) => {
            if let Some(val) = &args.browser_path {
                if val.trim().is_empty() {
                    eprintln!("Browser path cannot be empty.");
                    return Ok(());
                }

                match shared::app::config::update_browser_path(val, &mut config) {
                    Ok(_) => println!("Browser path updated!"),
                    Err(shared::app::config::ConfigError::InvalidPath(msg)) => {
                        eprint!("Invalid path: {}", msg);
                    }
                    Err(e) => {
                        eprint!("Error updating browser path: {:?}", e);
                    }
                }
                return Ok(());
            }
        }
        None => {
            if cli.run_cmd.is_empty() {
                println!("Run ui");
                std::process::Command::new("../ui/src-tauri/target/release/ui")
                    .spawn()
                    .expect("Failed to launch UI");
            } else {
                let name = &cli.run_cmd[0];
                match run::run(name.as_str(), &config) {
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
                }
            }
        }
    }

    Ok(())
}
