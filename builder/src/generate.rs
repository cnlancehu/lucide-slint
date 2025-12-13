use std::{
    fmt::Write as _,
    fs::{self},
    path::{Path, PathBuf},
};

use cnxt::Colorize;
use duct::cmd;
use tera::{Context, Tera};
use usvg::{Node, Options, Tree, WriteOptions, tiny_skia_path::PathSegment};

use crate::definition::{self, IconMetadata};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut template = Tera::default();
    template.add_raw_template("slint", include_str!("../slint.template"))?;

    let icons_dir_path = PathBuf::from("lucide/icons");
    let icons_process_dir_path = PathBuf::from("temp");
    let target_dir_path = PathBuf::from("lucide-slint");

    let _ = fs::remove_dir_all(&icons_process_dir_path);
    fs::create_dir_all(&icons_process_dir_path)?;

    preprocess_icons_svg(&icons_dir_path, &icons_process_dir_path)?;

    let icon_names = read_icon_names(&icons_process_dir_path)?;
    let icons =
        process_icons(&icons_dir_path, &icons_process_dir_path, icon_names)?;

    generate_slint_file(&template, &target_dir_path, &icons)?;

    println!(
        "{}",
        format!("Successfully transformed {} icons", icons.len())
            .bright_green()
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

fn preprocess_icons_svg(
    icons_dir_path: &Path,
    icons_process_dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let icons_dir = fs::read_dir(icons_dir_path)?;
    for entry in icons_dir {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.ends_with(".svg") {
            let svg_content = fs::read_to_string(entry.path())?;

            let optimized_svg_content =
                usvg::Tree::from_str(&svg_content, &Options::default())?;

            let output_path = icons_process_dir_path.join(&file_name);
            fs::write(
                output_path,
                optimized_svg_content.to_string(&WriteOptions::default()),
            )?;
        }
    }

    cmd!("pnpm.cmd", "start").dir("svgoptimize").run()?;

    Ok(())
}

fn process_icons(
    icons_dir_path: &Path,
    icons_process_dir_path: &Path,
    icon_names: Vec<String>,
) -> Result<Vec<definition::Icon>, Box<dyn std::error::Error>> {
    icon_names
        .into_iter()
        .map(|icon_name| {
            let icon_name_pascal =
                format!("{}Icon", to_pascal_case(&icon_name));
            let icon_raw_svg_filename = format!("{}.svg", &icon_name);
            let icon_metadata_filename = format!("{}.json", &icon_name);
            let icon_raw_svg = fs::read_to_string(
                icons_process_dir_path.join(&icon_raw_svg_filename),
            )?;
            let icon_svg_tree =
                Tree::from_str(&icon_raw_svg, &Options::default())?;
            let paths = process_icons_svg(&icon_svg_tree);
            if paths.len() > 1 {
                println!(
                    "{}",
                    format!(
                        "Warning: Icon '{}' has multiple paths ({} paths)",
                        icon_name,
                        paths.len()
                    )
                    .yellow()
                );
            }

            let icon_metadata =
                fs::read(icons_dir_path.join(icon_metadata_filename))?;
            let icon_metadata: IconMetadata =
                serde_json::from_slice(&icon_metadata)?;
            let deprecated = icon_metadata.deprecated;

            Ok(definition::Icon {
                name_pascal: icon_name_pascal,
                deprecated,
                paths,
            })
        })
        .collect()
}

fn process_icons_svg(tree: &Tree) -> Vec<definition::Path> {
    let size = tree.size();
    let width = size.width();
    let height = size.height();
    if width != height {
        eprintln!(
            "{}",
            format!(
                "Warning: SVG icon is not square (width: {}, height: {})",
                width, height
            )
            .yellow()
        );
    }
    let size = width;
    let mut paths: Vec<definition::Path> = Vec::new();

    tree.root().children().iter().for_each(|node| {
        if let Node::Path(path) = node {
            let data = path.data();
            let path_bounding_box = path.abs_bounding_box();
            let viewbox_x = -path_bounding_box.left() / size;
            let viewbox_y = -path_bounding_box.top() / size;
            let viewbox_width = width + viewbox_x.abs();
            let viewbox_height = height + viewbox_y.abs();
            let commands = path_segments_to_str(data).unwrap();
            paths.push(definition::Path {
                viewbox_x,
                viewbox_y,
                viewbox_width,
                viewbox_height,
                commands,
            });
        }
    });

    paths
}

fn path_segments_to_str(
    data: &usvg::tiny_skia_path::Path,
) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    for segment in data.segments() {
        match segment {
            PathSegment::MoveTo(p) => {
                s.write_fmt(format_args!("M {} {} ", p.x, p.y))?
            }
            PathSegment::LineTo(p) => {
                s.write_fmt(format_args!("L {} {} ", p.x, p.y))?
            }
            PathSegment::QuadTo(p0, p1) => s.write_fmt(format_args!(
                "Q {} {} {} {} ",
                p0.x, p0.y, p1.x, p1.y
            ))?,
            PathSegment::CubicTo(p0, p1, p2) => s.write_fmt(format_args!(
                "C {} {} {} {} {} {} ",
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y
            ))?,
            PathSegment::Close => s.write_fmt(format_args!("Z "))?,
        }
    }
    s.pop();

    Ok(s)
}

fn generate_slint_file(
    template: &Tera,
    target_dir_path: &Path,
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
