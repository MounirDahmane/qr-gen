use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HistoryEntry {
    pub text: String,
    pub ec_level: String,
}

pub fn save_history(history: &Vec<HistoryEntry>) {
    let path = dirs::data_dir()
        .unwrap_or_default()
        .join("qrgen_history.json");
    let json = serde_json::to_string(history).unwrap();
    std::fs::write(path, json).unwrap();
}

pub fn load_history() -> Vec<HistoryEntry> {
    let path = dirs::data_dir()
        .unwrap_or_default()
        .join("qrgen_history.json");
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
