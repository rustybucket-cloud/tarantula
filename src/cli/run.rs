use dirs;

use iced;

use crate::app::config;
use crate::app::install;
use crate::app::run;
use crate::app::uninstall;
use crate::app::update;
use crate::infra::app_data;
use crate::ui::ui::Ui;
use clap::{Args, Parser, Subcommand};

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

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let app_data_path = home_dir.join(".local/share/tarantula").to_path_buf();
    let desktop_data_path = home_dir.join(".local/share/applications").to_path_buf();
    let mut config = config::Config::new(app_data_path, desktop_data_path);
    config.browser_path = match config::get_browser_path(&config) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error retrieving browser path from config: {:?}", e);
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
            println!("Installed web apps:");
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
            println!("Update");
        }
        Some(Commands::Config(args)) => {
            if let Some(val) = &args.browser_path {
                if val.trim().is_empty() {
                    eprintln!("Browser path cannot be empty.");
                    return Ok(());
                }

                match crate::app::config::update_browser_path(val, &mut config) {
                    Ok(_) => println!("Browser path updated!"),
                    Err(crate::app::config::ConfigError::InvalidPath(msg)) => {
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
                match iced::application("Tarantula", Ui::update, Ui::view)
                    .run_with(move || (Ui::new(config.clone()), iced::Task::none()))
                {
                    Ok(_) => println!("Success!"),
                    Err(e) => eprintln!("{:?}", e),
                }
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
