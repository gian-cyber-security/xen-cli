use anyhow::Result;
use crate::{config, permissions};

pub fn command(cfg: &mut config::Config, args: Vec<&str>) -> Result<()> {
    if !permissions::allowed(cfg, "mcp") { println!("MCP access is disabled."); return Ok(()); }
    match args.first().copied().unwrap_or("list") {
        "list" => println!("No MCP servers registered yet."),
        "add" => println!("MCP add: paste a validated MCP JSON configuration in the next implementation pass."),
        "info" => println!("Usage: /mcp info <name>"),
        "enable" | "disable" | "remove" | "reload" => println!("MCP registry command accepted: {}", args[0]),
        _ => println!("/mcp add|list|info|enable|disable|remove|reload"),
    }
    Ok(())
}
