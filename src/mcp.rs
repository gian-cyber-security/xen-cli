use anyhow::{Context, Result};
use crate::{config, permissions};
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

pub fn command(cfg: &mut config::Config, args: Vec<&str>) -> Result<()> {
    if !permissions::allowed(cfg, "mcp") {
        println!("MCP access is disabled.");
        return Ok(());
    }

    match args.first().copied().unwrap_or("list") {
        "list" => list(cfg),
        "add" => add(cfg)?,
        "info" => info(cfg, args.get(1).copied())?,
        "enable" | "disable" => set_enabled(cfg, args.get(1).copied(), args[0] == "enable")?,
        "remove" => remove(cfg, args.get(1).copied())?,
        "reload" => println!("MCP stdio registry reloaded."),
        "run" => run(cfg, args.get(1).copied())?,
        _ => println!("/mcp add|list|info <name>|enable <name>|disable <name>|remove <name>|reload|run <name>"),
    }
    Ok(())
}

fn list(cfg: &config::Config) {
    if cfg.mcp_servers.is_empty() {
        println!("No MCP stdio servers registered.");
        return;
    }
    println!("NAME\tTRANSPORT\tSTATUS");
    for s in &cfg.mcp_servers {
        println!("{}\t{}\t{}", s.name, s.transport, if s.enabled {"ON"} else {"OFF"});
    }
}

fn add(cfg: &mut config::Config) -> Result<()> {
    println!("Paste MCP stdio JSON configuration.");
    println!("HTTP/SSE configurations are not supported yet.");
    print!("JSON: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let value: serde_json::Value =
        serde_json::from_str(input.trim()).map_err(|e| anyhow::anyhow!("Invalid MCP JSON: {e}"))?;

    let obj = value.as_object().context("MCP configuration must be a JSON object")?;
    let name = obj.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("mcp-server")
        .to_string();

    let command = obj.get("command")
        .and_then(|v| v.as_str())
        .context("stdio MCP JSON requires a string 'command' field")?;

    if command.is_empty() {
        anyhow::bail!("MCP command cannot be empty");
    }

    let transport = obj.get("transport")
        .and_then(|v| v.as_str())
        .unwrap_or("stdio");

    if transport != "stdio" {
        anyhow::bail!("Only MCP stdio transport is supported currently");
    }

    if obj.get("url").is_some() || obj.get("httpUrl").is_some() {
        anyhow::bail!("HTTP MCP is not supported currently");
    }

    cfg.mcp_servers.retain(|s| s.name != name);
    cfg.mcp_servers.push(config::McpServer {
        name: name.clone(),
        transport: "stdio".into(),
        enabled: true,
        config: value,
    });

    config::save(cfg)?;
    println!("MCP stdio server registered: {name}");
    println!("Command: {command}");
    Ok(())
}

fn info(cfg: &config::Config, name: Option<&str>) -> Result<()> {
    let name = name.context("Usage: /mcp info <name>")?;
    match cfg.mcp_servers.iter().find(|s| s.name == name) {
        Some(s) => {
            println!("Name: {}", s.name);
            println!("Transport: {}", s.transport);
            println!("Status: {}", if s.enabled {"ON"} else {"OFF"});
            if let Some(command) = s.config.get("command").and_then(|v| v.as_str()) {
                println!("Command: {command}");
            }
            if let Some(args) = s.config.get("args") {
                println!("Args: {}", args);
            }
        }
        None => println!("MCP server not found: {name}"),
    }
    Ok(())
}

fn set_enabled(cfg: &mut config::Config, name: Option<&str>, enabled: bool) -> Result<()> {
    let name = name.context("Usage: /mcp enable|disable <name>")?;
    match cfg.mcp_servers.iter_mut().find(|s| s.name == name) {
        Some(s) => {
            s.enabled = enabled;
            config::save(cfg)?;
            println!("{}: {}", name, if enabled {"enabled"} else {"disabled"});
        }
        None => println!("MCP server not found: {name}"),
    }
    Ok(())
}

fn remove(cfg: &mut config::Config, name: Option<&str>) -> Result<()> {
    let name = name.context("Usage: /mcp remove <name>")?;
    let before = cfg.mcp_servers.len();
    cfg.mcp_servers.retain(|s| s.name != name);
    config::save(cfg)?;

    if before == cfg.mcp_servers.len() {
        println!("MCP server not found.");
    } else {
        println!("MCP server removed: {name}");
    }
    Ok(())
}

fn run(cfg: &config::Config, name: Option<&str>) -> Result<()> {
    let name = name.context("Usage: /mcp run <name>")?;
    let server = cfg.mcp_servers.iter()
        .find(|s| s.name == name)
        .context("MCP server not found")?;

    if !server.enabled {
        anyhow::bail!("MCP server is disabled");
    }

    let command = server.config.get("command")
        .and_then(|v| v.as_str())
        .context("MCP stdio server has no command")?;

    let args: Vec<String> = server.config.get("args")
        .and_then(|v| v.as_array())
        .map(|items| items.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut child = spawn_stdio(command, &args, &server.config)?;
    println!("Started MCP stdio server '{name}' (pid {}).", child.id());
    println!("The process is kept attached to XEN CLI.");
    let status = child.wait()?;
    println!("MCP process exited with {status}");
    Ok(())
}

fn spawn_stdio(command: &str, args: &[String], config: &serde_json::Value) -> Result<Child> {
    let mut cmd = Command::new(command);
    cmd.args(args).stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());

    if let Some(env) = config.get("env").and_then(|v| v.as_object()) {
        for (key, value) in env {
            if let Some(value) = value.as_str() {
                cmd.env(key, value);
            }
        }
    }

    cmd.spawn().with_context(|| format!("Could not start MCP command: {command}"))
}
