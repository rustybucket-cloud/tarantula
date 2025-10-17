use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // Copy the UI binary to a known location relative to cli
    // UI is built in the workspace target directory
    let ui_binary_src = manifest_dir.join("../target/release/ui");
    let ui_binary_dest = manifest_dir.join("ui-binary");
    
    // Watch the UI binary itself, not just source files
    println!("cargo:rerun-if-changed=../target/release/ui");

    if !ui_binary_src.exists() {
        eprintln!("UI binary not found at {:?}", ui_binary_src);
        eprintln!("Please build the UI first: cd ui/src-tauri && cargo build --release");
        return Err("UI binary not found. Build it first with: cd ui/src-tauri && cargo build --release".into());
    }

    fs::copy(&ui_binary_src, &ui_binary_dest)?;
    println!(
        "cargo:warning=UI binary copied to {}",
        ui_binary_dest.display()
    );

    Ok(())
}
