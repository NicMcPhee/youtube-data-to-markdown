use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    kind: String,
    etag: String,
    id: String,
}

/// # Panics
/// 
/// This can panic if the JSON being read doesn't have the appropriate structure.
/// 
/// # Errors
/// 
/// This can return I/O errors from trying to open or read the file, and serde errors
/// from trying to parse the file. 
pub fn get_playlist_entries(path: &str) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;
    // If we can't get the `items` field, there was probably a problem in the
    // data, so we'll just die.
    #[allow(clippy::unwrap_used)]
    let items_json = json.get("items").unwrap().clone();
    let items = serde_json::from_value::<Vec<Entry>>(items_json)?;

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_entries() {
        let entries: Vec<Entry> = get_playlist_entries("data/playlist_entries.json").unwrap();
        assert_eq!(10, entries.len());
        assert_eq!("PLI9i5fpXEEc6_o2Xy0ozg_hrO4FgswkGG", entries[0].id);
    }
}