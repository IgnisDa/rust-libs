use anyhow::Result;
use arch_mirrors_rs::{Country, Status, Url};
use directories::BaseDirs;
use hashbag::HashBag;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn get_cache_file(name: Option<&str>) -> PathBuf {
    let name = name.unwrap_or("mirrorstatus.json");
    let base_dirs = BaseDirs::new().expect("is not expected to fail");
    let cache_dir = base_dirs.cache_dir();
    fs::create_dir_all(cache_dir).expect("creating directory should not fail");
    cache_dir.join(name)
}

/// Retrieve the mirror status JSON object. The downloaded data will be cached locally and
/// re-used within the cache timeout period. Returns the object and the local cache's
/// modification time.
pub async fn get_mirror_status(
    // TODO: Allow using this parameter
    connection_timeout: u8,
    cache_timeout: u8,
    url: &str,
    cache_file_path: &Path,
) -> Result<Status> {
    let mtime = cache_file_path
        .metadata()
        .ok()
        .and_then(|meta| meta.modified().ok());
    let is_invalid = mtime
        .map(|time| {
            let now = SystemTime::now();
            let elapsed = now.duration_since(time).expect("Time went backwards");
            elapsed.as_secs() > cache_timeout as u64
        })
        .unwrap_or(true);
    if !is_invalid {
        let loaded = serde_json::from_reader(File::open(cache_file_path).unwrap())?;
        Ok(loaded)
    } else {
        let loaded = Status::get_from_url(url).await?;
        let to_write = serde_json::to_string_pretty(&loaded).unwrap();
        fs::write(cache_file_path, to_write).expect("Writing to file should not fail");
        Ok(loaded)
    }
}

pub async fn count_countries(urls: &[Url]) -> HashBag<&Country> {
    let mut counts = HashBag::new();
    for url in urls.iter() {
        if url.country == Country::WORLDWIDE {
            continue;
        }
        counts.insert(&url.country);
    }
    counts
}
