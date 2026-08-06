use std::{
    fmt::Write as _,
    fs::{self, ReadDir},
    path::Path,
};

use cnxt::Colorize;
use tera::{Context, Tera};
use usvg::{Node, Options, Tree, tiny_skia_path::PathSegment};

use crate::{
    CRATE_PATH, LUCIDE_LAB_TEMPDIR, LUCIDE_TEMPDIR,
    definition::{self, Icon},
};

pub fn run_lucide() {
    let mut template = Tera::default();
    template
        .add_raw_template("lucide", include_str!("../lucide.template"))
        .unwrap();
    let files = fs::read_dir(LUCIDE_TEMPDIR).unwrap();
    let icons = generate_icons(files);

    let mut context = Context::new();
    context.insert("icons", &icons);
    let slint_file = template.render("lucide", &context).unwrap();
    fs::write(Path::new(CRATE_PATH).join("lucide.slint"), &slint_file).unwrap();

    fs::remove_dir_all(LUCIDE_TEMPDIR).unwrap();
}

pub fn run_lucide_lab() {
    let mut template = Tera::default();
    template
        .add_raw_template("lucide-lab", include_str!("../lucide-lab.template"))
        .unwrap();
    let files = fs::read_dir(LUCIDE_LAB_TEMPDIR).unwrap();
    let icons = generate_icons(files);

    let mut context = Context::new();
    context.insert("icons", &icons);
    let slint_file = template.render("lucide-lab", &context).unwrap();
    fs::write(Path::new(CRATE_PATH).join("lucide-lab.slint"), &slint_file)
        .unwrap();

    fs::remove_dir_all(LUCIDE_LAB_TEMPDIR).unwrap();
}

fn generate_icons(files: ReadDir) -> Vec<Icon> {
    let mut icons = Vec::new();
    for file in files
        .filter_map(|f| if let Ok(f) = f { Some(f) } else { None })
        .filter(|f| {
            f.file_type().and_then(|t| Ok(t.is_file())).is_ok_and(|r| r)
        })
    {
        if let Some(file_name) = file.path().file_name()
            && let Some(name) = file_name.to_str()
        {
            let content = fs::read_to_string(&file.path()).unwrap();
            let tree = Tree::from_str(&content, &Options::default()).unwrap();
            let paths = read_paths_from_tree(&tree);

            icons.push(Icon {
                name_pascal: name.to_string(),
                paths,
            });
        }
    }

    icons
}

fn read_paths_from_tree(tree: &Tree) -> Vec<definition::Path> {
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
            let has_fill = path.fill().is_some();
            paths.push(definition::Path {
                viewbox_x,
                viewbox_y,
                viewbox_width,
                viewbox_height,
                commands,
                has_fill,
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
