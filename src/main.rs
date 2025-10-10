use tarantula::cli::run;

fn main() {
    if let Err(e) = run::run() {
        eprint!("Error: {}", e);
        std::process::exit(1);
    }
}
