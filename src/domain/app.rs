use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub url: String,
}
