use color_eyre::Result;
use dinopkg_npm_registry::PackageInfo;

pub async fn install_cmd(name: String) -> Result<()> {
    let client = reqwest::Client::new();

    let package_info = PackageInfo::from_name(&name, &client).await?;
    dbg!(package_info);
    Ok(())
}
