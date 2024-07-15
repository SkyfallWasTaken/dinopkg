use std::env;

use camino::Utf8PathBuf;
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use dinopkg_package_json::PackageJson;
use gix_config::File as GitConfigFile;
use maplit::hashmap;
use owo_colors::OwoColorize;
use tokio::fs;

pub async fn init() -> Result<()> {
    // Get some project/env specific info to make the defaults more relevant
    let current_dir = Utf8PathBuf::try_from(env::current_dir()?)?;
    let current_dir_name = current_dir.file_name().unwrap_or("package");
    // FIXME: this blocks the event loop
    let git_config_file = GitConfigFile::from_git_dir(current_dir.join(".git").into());
    let git_repo_url = git_config_file
        .map(|config| {
            config
                .section("remote", Some("origin".into()))
                .ok()
                .and_then(|remote_section| {
                    remote_section
                        .body()
                        .value("url")
                        .map(|url| url.to_string())
                })
        })
        .ok()
        .flatten()
        .map(|url| url.replace("git@github.com", "https://github.com/"));

    // Now, onto the questions!
    let package_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package name")
        .default(current_dir_name.into())
        .interact_text()?;
    let version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package version")
        .default("1.0.0".into())
        .interact_text()?;
    let private = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is this a private package?")
        .default(true)
        .interact()?;
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Description")
        .allow_empty(true)
        .interact_text()?;
    let entry_point: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Entry point")
        .default("index.js".into())
        .interact_text()?;
    let test_command: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Test command")
        .default("echo \"Error: no test specified\" && exit 1".into())
        .interact_text()?;
    let git_repository: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Git repository")
        .default(git_repo_url.unwrap_or_default())
        .interact_text()?;
    let author: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author name")
        .allow_empty(true)
        .interact_text()?;
    let license: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("License")
        .default("MIT".into())
        .interact_text()?;

    let package_json = PackageJson {
        name: package_name,
        version,
        author: Some(author),
        repository: Some(git_repository),
        license: Some(license),
        description: Some(description),
        private,
        main: Some(entry_point),

        scripts: Some(hashmap! {
            "test".into() => test_command,
        }),

        dependencies: None,
        dev_dependencies: None,
    };
    let output = serde_json::to_string_pretty(&package_json)?;

    println!(
        "\nI will write the following output to {}:\n{output}\n",
        "`package.json`".purple()
    );

    let yes = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Is this okay?")
        .default(true)
        .interact()?;
    if yes {
        fs::write("package.json", output).await?;
        println!(
            "Successfully wrote new {} for {}!",
            "`package.json`".purple(),
            package_json.name.purple()
        );
    } else {
        println!("{}", "Cancelled.".bold().red())
    }

    Ok(())
}
