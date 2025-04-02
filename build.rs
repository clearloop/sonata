use anyhow::{anyhow, Result};
use std::{env, process::Command};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blog/theme");

    if env::var("THEME").is_err() {
        return Ok(());
    }

    // check if yarn exist
    let yarn = which::which("yarn");
    if yarn.is_ok() {
        // Build default theme
        if !Command::new(yarn?)
            .args(["--cwd", "theme", "build"])
            .status()?
            .success()
        {
            return Err(anyhow!("build default theme failed."));
        }
    }

    Ok(())
}
