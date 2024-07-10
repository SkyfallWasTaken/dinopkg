use std::{path::Path, process::ExitStatus};

use color_eyre::eyre::Result;
use tokio::process::Command;

pub const DEFAULT_SHELL: &str = if cfg!(windows) { "cmd.exe" } else { "/bin/sh" };
pub const DEFAULT_SHELL_EXEC_ARG: &str = if cfg!(windows) { "/c" } else { "-c" };

pub async fn run_script(
    shell: &str,
    shell_exec_arg: &str,
    command: &str,
    cwd_path: &Path,
) -> Result<ExitStatus> {
    // Scripts are run from the root of the package folder, regardless of what
    // the current working directory is when npm run is called. As such, we
    // do this too for compatibility (and also because it's ten times less annoying!)
    let mut tokio_command = Command::new(shell);
    debug_assert!(cwd_path.is_dir());
    tokio_command
        .arg(shell_exec_arg)
        .arg(command)
        .current_dir(cwd_path);
    Ok(tokio_command.status().await?)
}
