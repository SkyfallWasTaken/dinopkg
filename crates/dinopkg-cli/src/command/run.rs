use color_eyre::{eyre::eyre, Result};
use dinopkg_package_json::PackageJson;
use owo_colors::OwoColorize;

use crate::run_script::{run_script, DEFAULT_SHELL, DEFAULT_SHELL_EXEC_ARG};

pub async fn run(script_name: Option<String>) -> Result<()> {
    let (package_json, package_json_path) = PackageJson::from_file(10).await?;
    let root_path = package_json_path.parent().unwrap(); // Should never happen, `package.json` should always be there

    let Some(scripts) = package_json.scripts else {
        return Err(eyre!("no `scripts` provided in package.json"));
    };
    match script_name {
        Some(script_name) => {
            match scripts.get(&script_name) {
                Some(script) => {
                    println!("{} {}", "$".purple().dimmed(), script.bold().dimmed());

                    let status =
                        run_script(DEFAULT_SHELL, DEFAULT_SHELL_EXEC_ARG, &script, root_path)
                            .await?;

                    if cfg!(unix) {
                        use std::os::unix::process::ExitStatusExt;

                        if let Some(signal) = status.signal() {
                            return Err(eyre!(format!("process terminated by signal {signal}")));
                        }
                    }

                    // The only time the exit code isn't there is if the process was terminated by a signal.
                    // We check for that above (and on non-Unix systems, there will always be an exit code.)
                    let exit_code = status.code().unwrap();
                    if exit_code != exitcode::OK {
                        return Err(eyre!(format!("process exited with code {exit_code}")));
                    }
                }
                _ => return Err(eyre!(format!("script `{script_name}` not found"))),
            }
        }
        _ => {
            println!("{}", "Available scripts:".bold().underline());
            for (key, val) in scripts.iter() {
                println!("{} - {}", key.bold(), val.dimmed())
            }
        }
    }

    Ok(())
}
