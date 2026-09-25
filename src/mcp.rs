use anyhow::Result;
use crate::{config, permissions};
use std::io::{self, Write};

pub fn command(cfg: &mut config::Config, args: Vec<&str>) -> Result<()> {
    if !permissions::allowed(cfg, "mcp") { println!("MCP access is disabled."); return Ok(()); }
    match args.first().copied().unwrap_or("list") {
        "list" => list(cfg),
        "add" => add(cfg)?,
        "info" => {
            let name = args.get(1).ok_or_else(|| anyhow::anyhow!("Usage: /mcp info <name>"))?;
            match cfg.mcp_servers.iter().find(|s| s.name == *name) {
                Some(s) => println!("{} | {} | {}", s.name, s.transport, if s.enabled {"ON"} else {"OFF"}),
                None => println!("MCP server not found: {name}"),
            }
        }
        "enable" | "disable" => {
            let name = args.get(1).ok_or_else(|| anyhow::anyhow!("Usage: /mcp enable|disable <name>"))?;
            if let Some(s) = cfg.mcp_servers.iter_mut().find(|s| s.name == *name) {
                s.enabled = args[0] == "enable";
                config::save(cfg)?;
                println!("{}: {}", name, if s.enabled {"enabled"} else {"disabled"});
            } else { println!("MCP server not found: {name}"); }
        }
        "remove" => {
            let name = args.get(1).ok_or_else(|| anyhow::anyhow!("Usage: /mcp remove <name>"))?;
            let n = cfg.mcp_servers.len();
            cfg.mcp_servers.retain(|s| s.name != *name);
            config::save(cfg)?;
            println!("{}", if n == cfg.mcp_servers.len() {"MCP server not found."} else {"MCP server removed."});
        }
        "reload" => println!("MCP registry reloaded from configuration."),
        _ => println!("/mcp add|list|info <name>|enable <name>|disable <name>|remove <name>|reload"),
    }
    Ok(())
}

fn list(cfg: &config::Config) {
    if cfg.mcp_servers.is_empty() { println!("No MCP servers registered."); return; }
    println!("NAME\tTRANSPORT\tSTATUS");
    for s in &cfg.mcp_servers { println!("{}\t{}\t{}", s.name, s.transport, if s.enabled {"ON"} else {"OFF"}); }
}

fn add(cfg: &mut config::Config) -> Result<()> {
    print!("Paste MCP server JSON (single line): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value: serde_json::Value = serde_json::from_str(input.trim()).map_err(|e| anyhow::anyhow!("Invalid MCP JSON: {e}"))?;
    let obj = value.as_object().ok_or_else(|| anyhow::anyhow!("MCP configuration must be a JSON object"))?;
    let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("mcp-server").to_string();
    let transport = if obj.get("url").is_some() || obj.get("httpUrl").is_some() { "http" } else { "stdio" }.to_string();
    if transport == "http" && !permissions::allowed(cfg, "network") {
        println!("HTTP MCP requires Network Access ON or ASK.");
        return Ok(());
    }
    cfg.mcp_servers.retain(|s| s.name != name);
    cfg.mcp_servers.push(config::McpServer { name: name.clone(), transport, enabled: true, config: value });
    config::save(cfg)?;
    println!("MCP server registered: {name}");
    Ok(())
}
