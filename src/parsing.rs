use std::{fs::File, io::BufReader};

use serde_json::Value;

use crate::playlist::{Entry, Video};

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
mod entry_tests {
    use super::*;

    #[test]
    fn reads_entries() {
        let entries: Vec<Entry> = get_playlist_entries("data/playlist_entries.json").unwrap();
        assert_eq!(10, entries.len());
        assert_eq!("PLI9i5fpXEEc6_o2Xy0ozg_hrO4FgswkGG", entries[0].id);
    }
}

/// # Panics
/// 
/// This can panic if the JSON file doesn't have an `items` field.
/// 
/// # Errors
/// 
/// We can return an error if we can't open the file, or the file doesn't contain
/// legal JSON, or if the `items` field doesn't parse into a vector of `Video`s.
pub fn get_videos(path: &str) -> Result<Vec<Video>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;
    // If we can't get the `items` field, there was probably a problem in the
    // data, so we'll just die.
    #[allow(clippy::unwrap_used)]
    let items_json = json.get("items").unwrap().clone();
    let items = serde_json::from_value::<Vec<Video>>(items_json)?;

    Ok(items)
}

#[cfg(test)]
mod parsing_video_tests {
    use super::*;

    #[test]
    fn reads_videos() {
        let videos: Vec<Video> = get_videos("data/eps_page_1.json").unwrap();
        assert_eq!(50, videos.len());
        assert_eq!("VVU1dEdJUXRpMlVZZkNTSTlhVWVTWkZRLnc1dHhVcU5FYk1n", videos[0].id);
        // println!("{:?}", videos[0]);
    }
}
