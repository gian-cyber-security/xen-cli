use anyhow::Result;

use crate::{cli, config};

pub fn run() -> Result<()> {
    let mut cfg = config::load()?;
    println!("XEN CLI 0.2.0");
    println!("Local-first AI command line interface.");
    println!("Type /help for commands. Type /exit to quit.");
    cli::repl(&mut cfg)
}
