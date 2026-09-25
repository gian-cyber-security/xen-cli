mod app;
mod cli;
mod config;
mod hf;
mod mcp;
mod models;
mod permissions;
mod tools;

fn main() {
    if let Err(err) = app::run() {
        eprintln!("XEN CLI error: {err}");
        eprintln!("Type /help for help.");
        std::process::exit(1);
    }
}
