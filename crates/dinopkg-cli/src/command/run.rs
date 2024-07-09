use color_eyre::{eyre::eyre, Result};
use dinopkg_package_json::PackageJson;
use owo_colors::OwoColorize;

pub async fn run(script_name: Option<String>) -> Result<()> {
    let package_json = PackageJson::from_file(10).await?;

    match script_name {
        Some(script_name) => {
            let Some(scripts) = package_json.scripts else {
                return Err(eyre!("no `scripts` provided in package.json"));
            };
            match scripts.get(&script_name) {
                Some(script) => {
                    log::info!("{script_name}: {script}")
                }
                _ => return Err(eyre!(format!("script `{script_name}` not found"))),
            }
        }
        _ => {
            todo!()
        }
    }

    Ok(())
}
