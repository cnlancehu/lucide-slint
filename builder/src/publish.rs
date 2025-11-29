use std::path::Path;

use cnxt::Colorize;
use duct::cmd;
use serde::Deserialize;

use crate::generate;

const SUBMODULE_NAME: &str = "lucide";
const USER_AGENT: &str = "lucide-slint-builder";

#[derive(Debug, Deserialize)]
struct LatestRelease {
    tag_name: String,
}

#[derive(Debug, Deserialize)]
struct TagRefObject {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct TagRef {
    object: TagRefObject,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?;
    let release_url =
        "https://api.github.com/repos/lucide-icons/lucide/releases/latest";
    let release_res = client.get(release_url).send()?;
    let latest_release: LatestRelease = release_res.json()?;
    let version = latest_release.tag_name;
    let tag_ref_url = format!(
        "https://api.github.com/repos/lucide-icons/lucide/git/ref/tags/{}",
        version
    );
    let tag_res = client.get(&tag_ref_url).send()?;
    let tag_ref: TagRef = tag_res.json()?;
    let commit_id = tag_ref.object.sha;

    cmd!("git", "submodule", "update", "--init", "--recursive").run()?;
    cmd!("git", "-C", SUBMODULE_NAME, "checkout", &commit_id).run()?;

    println!(
        "{}",
        format!("Updated lucide submodule to version {}", version)
            .bright_green()
    );

    let cargo_toml_path = Path::new("./lucide-slint/Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(cargo_toml_path)?;
    let mut cargo_toml: toml_edit::DocumentMut = cargo_toml_content.parse()?;
    if let Some(version_entry) = cargo_toml
        .get_mut("package")
        .and_then(|pkg| pkg.get_mut("version"))
    {
        *version_entry = toml_edit::value(version);
        std::fs::write(cargo_toml_path, cargo_toml.to_string())?;
        generate::run()?;
    } else {
        return Err("Failed to find 'version' key in Cargo.toml".into());
    }

    println!("{}", "Finished, use `cargo publish -p lucide-slint --allow-dirty` to publish the new version.".bright_green());

    Ok(())
}
