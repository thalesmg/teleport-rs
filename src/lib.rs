use std::collections::HashMap;
use std::path::Path;

use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PortalConfig {
    pub portals: HashMap<String, String>,
}

impl PortalConfig {
    pub fn add_portal(&mut self, alias: &str, dst: &str) {
        self.portals.insert(alias.to_string(), dst.to_string());
    }

    pub fn del_portal(&mut self, alias: &str) {
        self.portals.remove(alias);
    }
}

pub fn teleport_to(path: String) {
    std::env::set_current_dir(path).unwrap();
}

pub fn load_portals(portals_path: &Path) -> Result<PortalConfig, Error> {
    let contents = std::fs::read_to_string(portals_path).unwrap_or("{\"portals\": {}}".to_string());
    let portals = serde_json::from_str(&contents)?;
    Ok(portals)
}

pub fn save_portals(portals_path: &Path, cfg: PortalConfig) -> Result<(), Error> {
    let blob = serde_json::to_string(&cfg)?;
    std::fs::write(portals_path, blob)?;
    Ok(())
}
