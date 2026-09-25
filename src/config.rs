use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Permission { On, Ask, Off }
impl Default for Permission { fn default() -> Self { Self::Ask } }

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub hf_api_key: Option<String>,
    pub permissions: Permissions,
    pub models: Vec<ModelEntry>,
    pub active_model: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub tools: Permission, pub files: Permission, pub commands: Permission,
    pub network: Permission, pub projects: Permission, pub models: Permission,
    pub mcp: Permission, pub shell: Permission,
}
impl Default for Permissions {
    fn default() -> Self { Self {
        tools: Permission::On, files: Permission::Ask, commands: Permission::Ask,
        network: Permission::Off, projects: Permission::Ask, models: Permission::On,
        mcp: Permission::Ask, shell: Permission::Ask,
    }}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelEntry { pub name:String, pub path:String, pub source:String, pub format:String }

pub fn path() -> Result<PathBuf> {
    Ok(dirs::config_dir().context("Could not find platform config directory")?.join("xen-cli").join("config.json"))
}
pub fn load() -> Result<Config> {
    let p=path()?; if !p.exists(){return Ok(Config::default());}
    Ok(serde_json::from_str(&fs::read_to_string(p)?)?)
}
pub fn save(c:&Config)->Result<()> {
    let p=path()?; if let Some(parent)=p.parent(){fs::create_dir_all(parent)?;}
    fs::write(p,serde_json::to_string_pretty(c)?)?; Ok(())
}
pub fn permission_mut(c:&mut Config,n:&str)->Option<&mut Permission>{match n{
"tools"=>Some(&mut c.permissions.tools),"files"=>Some(&mut c.permissions.files),
"commands"=>Some(&mut c.permissions.commands),"network"=>Some(&mut c.permissions.network),
"projects"=>Some(&mut c.permissions.projects),"models"=>Some(&mut c.permissions.models),
"mcp"=>Some(&mut c.permissions.mcp),"shell"=>Some(&mut c.permissions.shell),_=>None}}
