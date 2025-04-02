use anyhow::{anyhow, Result};
use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

/// Default theme files
const THEME: [&str; 3] = ["highlight.js", "highlight.css", "theme.css"];

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blog/theme");

    let res = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res");
    let yarn = which::which("yarn");
    if yarn.is_err() || std::env::var("SKIP_THEME").is_ok() {
        return build_no_yarn(&res);
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
        fs::copy(format!("{root}/blog/theme/{file}"), res.join(file))?;
    }

    Ok(())
}

/// Touch files if no yarn installed
fn build_no_yarn(out: &PathBuf) -> Result<()> {
    println!("cargo:warning=yarn not found, skip building default theme.");
    for file in THEME {
        File::create(out.join(file))?;
    }

    Ok(())
}
