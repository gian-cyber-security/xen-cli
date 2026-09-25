use anyhow::{Context, Result};
use std::process::{Command, Stdio};

pub fn execute(program: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("Could not start command: {program}"))?;
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    if !out.stderr.is_empty() {
        text.push_str(&String::from_utf8_lossy(&out.stderr));
    }
    if !out.status.success() {
        anyhow::bail!("Command exited with status {}", out.status);
    }
    Ok(text)
}

pub fn run_shell(command: &str) -> Result<String> {
    #[cfg(target_os = "windows")]
    { execute("cmd", &["/C", command]) }
    #[cfg(not(target_os = "windows"))]
    { execute("sh", &["-c", command]) }
}
