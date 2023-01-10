use anyhow::Result;
use arch_mirrors::Status;
use std::{fs::File, path::Path, time::SystemTime};

/// Retrieve the mirror status JSON object. The downloaded data will be cached locally and
/// re-used within the cache timeout period. Returns the object and the local cache's
/// modification time.
pub async fn get_mirror_status(
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
        Ok(Status::get_from_url(url).await?)
    }
}
