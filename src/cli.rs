use std::io::{self, Write};

pub fn run() {
    println!("XEN CLI v0.1.0");
    println!("Type /help for available commands.");
    println!();

    loop {
        print!("You > ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "/exit" || input == "/quit" {
            println!("Goodbye.");
            break;
        }

        handle_command(input);
    }
}

fn handle_command(input: &str) {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or("");

    match command {
        "/xen-cli" => show_center(),
        "/help" => help(),
        "/version" => println!("XEN CLI v0.1.0"),
        "/status" => println!("Status: READY"),
        "/models" => println!("No models registered yet. Use /scan or /insert <path>."),
        "/model" | "/use" => {
            match parts.next() {
                Some(model) => println!("Selected model: {model}"),
                None => usage("/use <model>", "/use DeepSeek-V4.1-Flash"),
            }
        }
        "/load" | "/insert" => {
            match parts.next() {
                Some(path) => println!("Local model path registered: {path}"),
                None => usage("/insert <local-model-path>", "/insert ./models/my-model"),
            }
        }
        "/add-hf-api" => println!("Hugging Face API setup is ready for implementation."),
        "/settings-xen" => settings(parts.collect()),
        "/mcp" => mcp(parts.collect()),
        "/scan" => println!("Scanning for supported local AI models..."),
        "/refresh" => println!("Model and tool registry refreshed."),
        "/unload" => println!("No model currently loaded."),
        "/ide" => println!("Checking supported IDEs..."),
        "/diagnostics" => println!("Diagnostics: basic CLI runtime OK."),
        "/tools" => println!("Tool access is configurable with /settings-xen."),
        "/config" | "/settings" => println!("Use /settings-xen for XEN CLI permissions."),
        "/exit" => {}
        _ => {
            println!("Unknown command: {command}");
            println!("Type /help to see available commands.");
        }
    }
}

fn show_center() {
    println!("XEN CLI v0.1.0");
    println!("Command Center");
    println!("Use /help for commands.");
}

fn help() {
    println!("Core: /xen-cli /version /status /models /use /load /insert /scan");
    println!("HF: /add-hf-api /hf");
    println!("Tools: /tools /project /files /read /voice /vision");
    println!("Permissions: /settings-xen");
    println!("MCP: /mcp");
    println!("System: /diagnostics /ide /exit /uninstall-xen-cli");
}

fn usage(syntax: &str, example: &str) {
    println!("Usage: {syntax}");
    println!("Example: {example}");
}

fn settings(args: Vec<&str>) {
    if args.is_empty() {
        println!("XEN CLI SETTINGS");
        println!("Tool Access       : ON");
        println!("File Access       : ASK");
        println!("Command Execution : ASK");
        println!("Network Access    : OFF");
        println!("Project Editing   : ASK");
        println!("Local AI Models   : ON");
        println!("MCP               : ASK");
        println!("Shell             : ASK");
        return;
    }
    if args[0] == "reset" {
        println!("XEN CLI settings reset.");
        return;
    }
    if args.len() == 2 && ["on", "ask", "off"].contains(&args[1]) {
        println!("Setting '{}' changed to {}.", args[0], args[1].to_uppercase());
    } else {
        println!("Usage: /settings-xen <tools|files|commands|network|projects|models|mcp|shell> <on|ask|off>");
    }
}

fn mcp(args: Vec<&str>) {
    match args.first().copied() {
        None => println!("MCP commands: /mcp add, /mcp list, /mcp info <name>, /mcp enable <name>, /mcp disable <name>, /mcp remove <name>, /mcp reload"),
        Some("add") => println!("Paste your MCP server JSON configuration."),
        Some("list") => println!("No MCP servers registered."),
        Some("reload") => println!("MCP registry reloaded."),
        _ => println!("Unknown MCP command. Type /mcp for help."),
    }
}
