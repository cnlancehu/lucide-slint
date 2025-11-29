use cnxt::Colorize;

mod definition;
mod generate;
mod publish;

fn main() {
    let arg = std::env::args().nth(1);
    match arg {
        Some(ref s) if s == "generate" => {
            if let Err(e) = generate::run() {
                eprintln!("{}", e.to_string().red());
                std::process::exit(1);
            }
        }
        Some(ref s) if s == "publish" => {
            if let Err(e) = publish::run() {
                eprintln!("{}", e.to_string().red());
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("{}", "Usage: cargo run -- generate".red());
            eprintln!("{}", "       cargo run -- update".red());
            std::process::exit(1);
        }
    }
}
