use anyhow::Result;
use std::io::{self, Write};

use crate::{config, diagnostics, hf, ide, mcp, models, permissions, project, runtime, tools};

pub fn repl(cfg: &mut config::Config) -> Result<()> {
    loop {
        print!("xen> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 { break; }
        let line = line.trim();
        if line.is_empty() { continue; }
        if line == "/exit" || line == "/quit" { break; }

        if let Err(e) = dispatch(cfg, line) {
            println!("Error: {e}");
        }
    }
    config::save(cfg)?;
    Ok(())
}

fn dispatch(cfg: &mut config::Config, line: &str) -> Result<()> {
    let mut p = line.split_whitespace();
    let cmd = p.next().unwrap_or("");
    match cmd {
        "/help" => help(),
        "/xen-cli" | "/status" => status(cfg),
        "/version" => println!("XEN CLI 0.2.0"),
        "/models" => list_models(cfg),
        "/scan" => scan_models(cfg),
        "/refresh" => scan_models(cfg),
        "/insert" => {
            let path = p.next().ok_or_else(|| anyhow::anyhow!("Usage: /insert <local-model-path>"))?;
            let (name, format) = models::insert(path)?;
            cfg.models.push(config::ModelEntry {
                name: name.clone(), path: path.into(), source: "local".into(), format
            });
            config::save(cfg)?;
            println!("Registered local model: {name}");
        }
        "/use" => {
            let name = p.next().ok_or_else(|| anyhow::anyhow!("Usage: /use <model>"))?;
            if cfg.models.iter().any(|m| m.name == name) {
                cfg.active_model = Some(name.into());
                config::save(cfg)?;
                println!("Active model: {name}");
            } else {
                println!("Model not registered. Use /scan or /insert <path>.");
            }
        }
        "/load" => {
            let path = p.next().ok_or_else(|| anyhow::anyhow!("Usage: /load <path>"))?;
            let (name, format) = models::insert(path)?;
            cfg.models.retain(|m| m.name != name);
            cfg.models.push(config::ModelEntry {
                name: name.clone(), path: path.into(), source: "session".into(), format
            });
            cfg.active_model = Some(name.clone());
            config::save(cfg)?;
            println!("Loaded for this configuration: {name}");
        }
        "/unload" => { cfg.active_model = None; config::save(cfg)?; println!("No active model."); }
        "/add-hf-api" => {
            let key = p.next().ok_or_else(|| anyhow::anyhow!("Usage: /add-hf-api <token>"))?;
            cfg.hf_api_key = Some(key.into());
            config::save(cfg)?;
            println!("Hugging Face API configured.");
        }
        "/hf" => {
            let sub = p.next().unwrap_or("help");
            match sub {
                "search" => hf::search(cfg, &p.collect::<Vec<_>>().join(" "))?,
                "info" => hf::info(cfg, p.next().ok_or_else(|| anyhow::anyhow!("Usage: /hf info <model>"))?)?,
                "install" => hf::install(cfg, p.next().ok_or_else(|| anyhow::anyhow!("Usage: /hf install <model>"))?)?,
                "remove" => hf::remove(cfg, p.next().ok_or_else(|| anyhow::anyhow!("Usage: /hf remove <model>"))?)?,
                _ => println!("/hf search <query> | /hf info <model> | /hf install <model> | /hf remove <model>"),
            }
        }
        "/read" => {
            let path = p.next().ok_or_else(|| anyhow::anyhow!("Usage: /read <file>"))?;
            if permissions::allowed(cfg, "files") { println!("{}", tools::read_file(path)?); }
        }
        "/files" => {
            let path = p.next().unwrap_or(".");
            if permissions::allowed(cfg, "files") { tools::list_files(path)?; }
        }
        "/settings-xen" => settings(cfg, p.collect()),
        "/mcp" => mcp::command(cfg, p.collect()),
        "/tools" => println!("Tool engine: {} (permissions control access)", if permissions::allowed(cfg, "tools") {"ON"} else {"OFF"}),
        "/chat" => {
            let prompt = p.collect::<Vec<_>>().join(" ");
            if prompt.is_empty() { println!("Usage: /chat <prompt>"); }
            else if permissions::allowed(cfg, "tools") { crate::inference::chat(cfg, &prompt)?; }
        },
        "/project" => { let path = p.next().unwrap_or("."); if permissions::allowed(cfg, "projects") { project::inspect(path)?; } },
        "/ide" => ide::detect(),
        "/diagnostics" => diagnostics::run(cfg),
        "/shell" => { let command = p.collect::<Vec<_>>().join(" "); if command.is_empty() { println!("Usage: /shell <command>"); } else if permissions::allowed(cfg, "shell") { println!("{}", runtime::run_shell(&command)?); } },
        "/config" => println!("{}", serde_json::to_string_pretty(cfg)?),
        _ => println!("Unknown command: {cmd}. Type /help."),
    }
    Ok(())
}

fn help() {
    println!("Core: /xen-cli /help /version /status /models /scan /refresh");
    println!("Models: /insert <path> /load <path> /use <model> /unload /chat <prompt>");
    println!("Files: /files [path] /read <file> /project [path]");
    println!("Hugging Face: /add-hf-api /hf search|info|install|remove");
    println!("MCP: /mcp add|list|info|enable|disable|remove|reload");
    println!("Settings: /settings-xen [permission] [on|ask|off] /settings-xen reset");
    println!("Session: /config /tools /ide /diagnostics /shell /exit");
}

fn status(cfg: &config::Config) {
    println!("XEN CLI 0.2.0");
    println!("Active model: {}", cfg.active_model.as_deref().unwrap_or("none"));\n    println!("Platform: {} / {}", std::env::consts::OS, std::env::consts::ARCH);
    println!("Registered models: {}", cfg.models.len());
    println!("HF API: {}", if cfg.hf_api_key.is_some() {"configured"} else {"not configured"});
}

fn list_models(cfg: &config::Config) {
    if cfg.models.is_empty() { println!("No models registered."); return; }
    for m in &cfg.models { println!("{} | {} | {} | {}", m.name, m.source, m.format, m.path); }
}

fn scan_models(cfg: &mut config::Config) {
    for (name, path, format) in models::scan() {
        if !cfg.models.iter().any(|m| m.path == path) {
            cfg.models.push(config::ModelEntry { name, path, source: "scan".into(), format });
        }
    }
    let _ = config::save(cfg);
    println!("Scan complete. {} models registered.", cfg.models.len());
}

fn settings(cfg: &mut config::Config, args: Vec<&str>) {
    if args.is_empty() {
        println!("Use: /settings-xen <tools|files|commands|network|projects|models|mcp|shell> <on|ask|off>");
        println!("Or: /settings-xen reset");
        return;
    }
    if args[0] == "reset" {
        cfg.permissions = config::Permissions::default();
        let _ = config::save(cfg);
        println!("Permissions reset.");
        return;
    }
    if args.len() < 2 { println!("Usage: /settings-xen <permission> <on|ask|off>"); return; }
    let Some(slot) = config::permission_mut(cfg, args[0]) else { println!("Unknown permission."); return; };
    *slot = match args[1] {
        "on" => config::Permission::On,
        "ask" => config::Permission::Ask,
        "off" => config::Permission::Off,
        _ => { println!("Mode must be on, ask, or off."); return; }
    };
    let _ = config::save(cfg);
    println!("Updated {} -> {}", args[0], args[1]);
}
