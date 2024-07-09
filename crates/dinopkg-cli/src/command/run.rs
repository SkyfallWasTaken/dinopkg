use color_eyre::Result;
use dinopkg_package_json::PackageJson;

pub async fn run(script_name: Option<String>) -> Result<()> {
    let package_json = PackageJson::from_file(10).await?;
    println!("Running script: {:?} {}", script_name, package_json.name);
    Ok(())
}
