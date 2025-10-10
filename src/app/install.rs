pub enum InstallError {
    Io(std::io::Error),
}

pub fn install(name: &str, url: &str) -> Result<(), InstallError> {
    println!("Name: {}, Url: {}", name, url);

    Ok(())
}
