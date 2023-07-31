use std::{fs, path::PathBuf};

use bimap::BiHashMap;
use tracing::{event, Level};

use crate::common::{Links, DEFAULT_STORE_FILENAME};

pub fn get_store_filename() -> PathBuf {
    PathBuf::from(std::env::var("STORE_FILENAME").unwrap_or(DEFAULT_STORE_FILENAME.to_string()))
}

pub fn load() -> Links {
    let result = || -> anyhow::Result<Links> {
        let store_filename = get_store_filename();

        let contents = fs::read_to_string(store_filename)?;
        event!(Level::TRACE, "Finished reading from disk");

        let map: Links = serde_json::from_str(&contents)?;
        event!(Level::TRACE, "Finished deserializing");
        Ok(map)
    }();

    match result {
        Ok(links) => {
            event!(
                Level::INFO,
                "Loaded {} link{} from disk",
                links.len(),
                if links.len() == 1 { "" } else { "s" }
            );
            links
        }
        Err(err) => {
            event!(Level::WARN, "Failed to load links from disk: {}", err);
            BiHashMap::new()
        }
    }
}

pub fn save(links: &Links) {
    let result = || -> anyhow::Result<()> {
        let store_filename = get_store_filename();

        let contents = serde_json::to_string_pretty(links)?;
        event!(Level::TRACE, "Finished serializing");

        fs::write(&store_filename, contents)?;
        event!(Level::TRACE, "Finished writing to disk");
        event!(
            Level::INFO,
            "Saved {} link{} to disk",
            links.len(),
            if links.len() == 1 { "" } else { "s" }
        );
        Ok(())
    }();

    if let Err(err) = result {
        event!(Level::WARN, "Failed to save links to disk: {}", err)
    }
}
