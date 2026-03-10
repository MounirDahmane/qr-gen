use serde::{Deserialize, Serialize};

/// A single saved QR entry. `ec_level` is stored as a string ("L", "M", "Q", "H")
/// because `qrcode::EcLevel` doesn't implement serde traits.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HistoryEntry {
    pub text: String,
    pub ec_level: String,
}

/// Serializes the full history to a JSON file in the OS data directory.
/// Overwrites the file on every call — no appending.
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
    // If the file doesn't exist yet or fails to parse, silently return empty —
    // this is expected on first launch.
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
