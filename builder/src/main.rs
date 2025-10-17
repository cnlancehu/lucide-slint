use std::{
    fs::{self},
    path::PathBuf,
};

use cnxt::Colorize;
use tera::{Context, Tera};

use crate::definition::IconMetadata;

mod definition;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.to_string().red());
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut template = Tera::default();
    template.add_raw_template("slint", include_str!("../slint.template"))?;

    let icons_dir_path = PathBuf::from("lucide/icons");
    let target_dir_path = PathBuf::from("lucide-slint");
    let icons_target_dir_path = target_dir_path.join("icons");
    let _ = fs::remove_dir_all(&icons_target_dir_path);
    fs::create_dir_all(&icons_target_dir_path)?;

    // Create target directory if it doesn't exist
    fs::create_dir_all(&icons_target_dir_path)?;

    let icon_names = read_icon_names(&icons_dir_path)?;
    let icons =
        process_icons(&icons_dir_path, &icons_target_dir_path, icon_names)?;

    generate_slint_file(&template, &target_dir_path, &icons)?;

    println!(
        "{}",
        format!("Successfully generated {} icons", icons.len()).bright_green()
    );
    Ok(())
}

fn read_icon_names(
    icons_dir_path: &PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let icons_dir = fs::read_dir(icons_dir_path).map_err(|e| {
        format!(
            "{}\n{}\n{}",
            "Failed to read icons directory from `lucide/icons`",
            "Make sure you have cloned the lucide repository and run this program from the root directory of the workspace.",
            e
        )
    })?;

    let mut icon_names: Vec<String> = icons_dir
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(|s| s.strip_suffix(".svg"))
                .map(String::from)
        })
        .collect();

    icon_names.sort_unstable();
    icon_names.dedup();

    Ok(icon_names)
}

fn process_icons(
    icons_dir_path: &PathBuf,
    icons_target_dir_path: &PathBuf,
    icon_names: Vec<String>,
) -> Result<Vec<definition::Icon>, Box<dyn std::error::Error>> {
    icon_names
        .into_iter()
        .map(|icon_name| {
            let icon_name_pascal =
                format!("{}Icon", to_pascal_case(&icon_name));
            let icon_raw_svg_filename = format!("{}.svg", &icon_name);
            let icon_metadata_filename = format!("{}.json", &icon_name);
            let url = format!("icons/{}", &icon_raw_svg_filename);
            let icon_raw_svg = fs::read_to_string(
                icons_dir_path.join(&icon_raw_svg_filename),
            )?;

            let icon_metadata =
                fs::read(icons_dir_path.join(icon_metadata_filename))?;
            let icon_metadata: IconMetadata =
                serde_json::from_slice(&icon_metadata)?;
            let deprecated = icon_metadata.deprecated;

            fs::write(
                icons_target_dir_path.join(&icon_raw_svg_filename),
                &icon_raw_svg,
            )?;

            Ok(definition::Icon {
                name_pascal: icon_name_pascal,
                url,
                deprecated,
            })
        })
        .collect()
}

fn generate_slint_file(
    template: &Tera,
    target_dir_path: &PathBuf,
    icons: &[definition::Icon],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("icons", icons);

    let slint_file = template.render("slint", &context)?;
    fs::write(target_dir_path.join("lib.slint"), &slint_file)?;

    Ok(())
}

/// Converts a string like "a-arrow-down" or "arrow_left_right" into PascalCase ("AArrowDown")
fn to_pascal_case(s: &str) -> String {
    s.split(['-', '_', ' '])
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect()
}
