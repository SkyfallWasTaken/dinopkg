use color_eyre::Result;
use dinopkg_npm_registry::PackageInfo;
use owo_colors::OwoColorize;

pub async fn install_cmd(name: String) -> Result<()> {
    let client = reqwest::Client::new();

    let package_info = PackageInfo::from_name(&name, &client).await?;
    let latest_version = package_info.dist_keys.get("latest").unwrap();
    let latest_package = package_info.versions.get(latest_version).unwrap();
    print_dep_version(&latest_package.name, &latest_package.version, false);

    if let Some(deps) = &latest_package.dependencies {
        for (dep_name, dep_version) in deps {
            print_dep_version(dep_name, dep_version, false);
        }
    }
    if let Some(deps) = &latest_package.dev_dependencies {
        for (dep_name, dep_version) in deps {
            print_dep_version(dep_name, dep_version, true);
        }
    }

    Ok(())
}

fn print_dep_version(name: &String, version: &String, is_dev: bool) {
    println!(
        "  {} {} {}{}",
        "Installing".green().bold(),
        name,
        version,
        if is_dev {
            format!("{}", " (dev)".dimmed().bold())
        } else {
            "".into()
        }
    );
}
