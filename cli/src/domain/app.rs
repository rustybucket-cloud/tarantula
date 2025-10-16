use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
}
