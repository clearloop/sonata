use anyhow::{anyhow, Result};
use std::{fs::File, path::PathBuf, process::Command};

/// Default theme files
const THEME: [&str; 3] = ["highlight.js", "highlight.css", "theme.css"];

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blog/theme");

    // check if yarn exist
    let root = env!("CARGO_MANIFEST_DIR");
    let out = PathBuf::from(root).join("blog/out");
    let yarn = which::which("yarn");
    if yarn.is_err() {
        return build_no_yarn(&out, false);
    }

    // check if THEME is enabled
    if std::env::var("THEME").is_err() {
        return build_no_yarn(&out, true);
    }

    // Build default theme
    if !Command::new(yarn?)
        .args(["--cwd", "theme", "build"])
        .status()?
        .success()
    {
        return Err(anyhow!("build default theme failed."));
    }

    Ok(())
}

/// Touch files if no yarn installed
fn build_no_yarn(out: &PathBuf, yarn: bool) -> Result<()> {
    if yarn {
        println!("THEME is disabled, set THEME=1 to enable default theme.");
    } else {
        println!("cargo:warning=yarn not found.");
    }
    for file in THEME {
        File::create(out.join(file))?;
    }

    Ok(())
}
