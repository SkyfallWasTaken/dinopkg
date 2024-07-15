use std::env;
use std::path::PathBuf;

use crate::Error;

#[cfg(feature = "tokio")]
pub async fn find_package_json(max_attempts: usize) -> Result<Option<PathBuf>, Error> {
    use tokio::fs;

    let mut current_dir = env::current_dir()?;
    for _ in 0..max_attempts {
        let package_json_path = current_dir.join("package.json");
        if fs::metadata(&package_json_path).await.is_ok() {
            return Ok(Some(package_json_path));
        }
        if !current_dir.pop() {
            break;
        }
    }
    Ok(None)
}
