use anyhow::{anyhow, Result};
use std::{
    fs::{self, File},
    process::Command,
};

/// Default theme files
const THEME: [&str; 3] = ["highlight.js", "highlight.css", "theme.css"];

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blog/theme");

    let out = std::env::var("OUT_DIR")?;
    let yarn = which::which("yarn");
    if yarn.is_err() || std::env::var("THEME").is_err() {
        return build_no_yarn(&out);
    }

    // Build default theme
    if !Command::new(yarn?)
        .args(["--cwd", "theme", "build"])
        .status()?
        .success()
    {
        return Err(anyhow!("build default theme failed."));
    }

    // Copy theme to output directory
    let root = env!("CARGO_MANIFEST_DIR");
    for file in THEME {
        fs::copy(format!("{root}/blog/theme/{file}"), format!("{out}/{file}"))?;
    }

    Ok(())
}

/// Touch files if no yarn installed
fn build_no_yarn(out: &str) -> Result<()> {
    println!("cargo:warning=skip building default theme, set THEME=1 to enable it.");
    for file in THEME {
        File::create(format!("{out}/{file}"))?;
    }

    Ok(())
}
