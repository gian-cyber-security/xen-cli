use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn inspect(path: &str) -> Result<()> {
    let root = Path::new(path);
    let meta = fs::metadata(root).with_context(|| format!("Could not access project: {path}"))?;
    if !meta.is_dir() { println!("Project path is a file: {}", root.display()); return Ok(()); }
    println!("Project: {}", root.display());
    let mut count = 0usize;
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        println!("  {}", entry.file_name().to_string_lossy());
        count += 1;
        if count >= 40 { println!("  ..."); break; }
    }
    Ok(())
}
