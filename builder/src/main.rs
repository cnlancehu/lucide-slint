use cnxt::Colorize;

mod definition;
mod step1;
mod step3;

pub const LUCIDE_TEMPDIR: &str = "./temp/lucide";
pub const LUCIDE_LAB_TEMPDIR: &str = "./temp/lab";

pub const LUCIDE_SOURCEDIR: &str = "./node_modules/lucide-static/icons";
pub const LUCIDE_LAB_SOURCEDIR: &str = "./node_modules/@lucide/lab/icons";

pub const CRATE_PATH: &str = "./lucide-slint";

fn main() {
    let arg = std::env::args().nth(1);
    match arg {
        Some(ref s) if s == "step1" => {
            step1::run(LUCIDE_SOURCEDIR, LUCIDE_TEMPDIR);
            step1::run(LUCIDE_LAB_SOURCEDIR, LUCIDE_LAB_TEMPDIR);
        }
        Some(ref s) if s == "step3" => {
            step3::run_lucide();
            step3::run_lucide_lab();
        }
        _ => {
            eprintln!("{}", "Usage: cargo run -- step1".red());
            eprintln!("{}", "       cargo run -- step3".red());
            std::process::exit(1);
        }
    }
}
