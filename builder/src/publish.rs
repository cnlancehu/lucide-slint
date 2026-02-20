use std::{env, fs::OpenOptions, io::Write, path::Path, process::exit};

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

#[derive(Debug, Deserialize)]
struct GitTagObject {
    #[serde(rename = "type")]
    object_type: String,
    sha: String,
}

#[derive(Debug, Deserialize)]
struct GitTag {
    object: GitTagObject,
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

    // The ref may point to an annotated tag object; dereference until we reach a commit.
    let mut commit_id = tag_ref.object.sha;
    loop {
        let tag_obj_url = format!(
            "https://api.github.com/repos/lucide-icons/lucide/git/tags/{}",
            commit_id
        );
        let tag_obj_res = client.get(&tag_obj_url).send()?;

        // If this isn't a tag object (e.g. it's already a commit SHA), stop dereferencing.
        if !tag_obj_res.status().is_success() {
            break;
        }

        let tag_obj: GitTag = tag_obj_res.json()?;
        if tag_obj.object.object_type == "commit" {
            commit_id = tag_obj.object.sha;
            break;
        }

        // Nested annotated tag
        commit_id = tag_obj.object.sha;
    }

    cmd!("git", "submodule", "update", "--init", "--recursive").run()?;
    cmd!(
        "git",
        "-C",
        SUBMODULE_NAME,
        "fetch",
        "--tags",
        "--force",
        "--prune",
        "origin"
    )
    .run()?;
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
        if version_entry.as_str() == Some(&version) {
            println!(
                "{}",
                format!(
                    "Cargo.toml already has version {}, skipping update",
                    version
                )
                .bright_yellow()
            );
            exit(11);
        }

        if let Ok(github_env_path) = env::var("GITHUB_ENV") {
            #[cfg(windows)]
            cmd!("pnpm.cmd", "install").dir("svgoptimize").run()?;

            #[cfg(not(windows))]
            cmd!("pnpm", "install").dir("svgoptimize").run()?;

            let mut file =
                OpenOptions::new().append(true).open(github_env_path)?;
            writeln!(file, "LUCIDE_VERSION={}", version)
                .expect("fAiled to write LUCIDE_VERSION to GITHUB_ENV");
        }

        *version_entry = toml_edit::value(version);
        std::fs::write(cargo_toml_path, cargo_toml.to_string())?;
        generate::run()?;
    } else {
        return Err("Failed to find 'version' key in Cargo.toml".into());
    }

    println!("{}", "Finished, use `cargo publish -p lucide-slint --allow-dirty` to publish the new version.".bright_green());

    Ok(())
}
