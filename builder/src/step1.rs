use std::{fs, path::Path};

use usvg::{Options, WriteOptions};

pub fn run(source: &str, destination: &str) {
    let destination = Path::new(destination);
    fs::create_dir_all(destination).unwrap();

    let files = fs::read_dir(source).unwrap();
    for file in files
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().map(|t| t.is_file()).is_ok_and(|r| r))
    {
        if let Some(file_name) = file.path().file_name()
            && let Some(file_name) = file_name.to_str()
            && let Some(name) = file_name.strip_suffix(".svg")
        {
            let content = fs::read_to_string(file.path()).unwrap();
            let content =
                usvg::Tree::from_str(&content, &Options::default()).unwrap();
            let content = content.to_string(&WriteOptions::default());
            fs::write(destination.join(to_pascal_case(name)), content).unwrap();
        }
    }
}

pub fn to_pascal_case(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut uppercase_next = true;

    for ch in input.chars() {
        if ch.is_whitespace() || ch == '-' || ch == '_' {
            uppercase_next = true;
        } else if uppercase_next {
            output.extend(ch.to_uppercase());
            uppercase_next = false;
        } else {
            output.push(ch);
        }
    }

    output
}
