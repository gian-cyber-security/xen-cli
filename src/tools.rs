use anyhow::{Context,Result};use std::{fs,path::Path};
pub fn read_file(p:&str)->Result<String>{Ok(fs::read_to_string(Path::new(p)).with_context(||format!("Could not read {p}"))?)}
pub fn list_files(p:&str)->Result<()>{for e in fs::read_dir(p).with_context(||format!("Could not list {p}"))?{println!("{}",e?.path().display());}Ok(())}
