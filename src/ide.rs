use std::path::Path;

pub fn detect() {
    let candidates = if cfg!(target_os = "windows") {
        vec!["Code.exe", "antigravity.exe"]
    } else if cfg!(target_os = "macos") {
        vec!["/Applications/Visual Studio Code.app", "/Applications/Antigravity.app"]
    } else {
        vec!["code", "antigravity"]
    };
    for c in candidates {
        let found = if c.contains('/') { Path::new(c).exists() } else { command_exists(c) };
        println!("{}: {}", c, if found {"detected"} else {"not found"});
    }
}

fn command_exists(name: &str) -> bool {
    std::env::var_os("PATH").map(|p| {
        std::env::split_paths(&p).any(|dir| dir.join(name).is_file())
    }).unwrap_or(false)
}
