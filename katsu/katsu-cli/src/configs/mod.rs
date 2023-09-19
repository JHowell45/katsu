use serde::{Deserialize, Serialize};
use confy;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct CliConfig {
    version: u8,
    ssh_private_key_path: String,
}

impl CliConfig {
    pub fn load() -> Self {
        confy::load("katsu-cli", None).unwrap()
    }
}