use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;

fn key(c: &crate::config::Config) -> Result<&str> {
    c.hf_api_key.as_deref().context("Hugging Face API is not configured. Use /add-hf-api first.")
}

pub fn search(c: &crate::config::Config, q: &str) -> Result<()> {
    let u = format!("https://huggingface.co/api/models?search={}", urlencoding::encode(q));
    let v: Vec<Value> = Client::new().get(u).bearer_auth(key(c)?).send()?.error_for_status()?.json()?;
    for x in v.iter().take(10) { println!("{}", x.get("id").and_then(Value::as_str).unwrap_or("unknown")); }
    Ok(())
}

pub fn info(c: &crate::config::Config, m: &str) -> Result<()> {
    let u = format!("https://huggingface.co/api/models/{m}");
    let v: Value = Client::new().get(u).bearer_auth(key(c)?).send()?.error_for_status()?.json()?;
    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}

pub fn install(c: &mut crate::config::Config, m: &str) -> Result<()> {
    let u = format!("https://huggingface.co/api/models/{m}");
    let v: Value = Client::new().get(u).bearer_auth(key(c)?).send()?.error_for_status()?.json()?;
    let id = v.get("id").and_then(Value::as_str).unwrap_or(m).to_string();
    let root = dirs::data_dir().context("Could not find data directory")?.join("xen-cli").join("hf");
    fs::create_dir_all(&root)?;
    let path = root.join(id.replace('/', "__"));
    fs::write(&path, serde_json::to_string_pretty(&v)?)?;
    c.models.retain(|x| x.name != id);
    c.models.push(crate::config::ModelEntry { name: id.clone(), path: path.display().to_string(), source: "huggingface".into(), format: "HF metadata".into() });
    crate::config::save(c)?;
    println!("Registered Hugging Face model: {id}");
    println!("Model weights are not downloaded automatically.");
    Ok(())
}

pub fn remove(c: &mut crate::config::Config, m: &str) -> Result<()> {
    let before = c.models.len();
    c.models.retain(|x| x.name != m);
    crate::config::save(c)?;
    println!("{}", if before == c.models.len() {"Model not registered."} else {"Model removed from registry."});
    Ok(())
}
