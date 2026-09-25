use anyhow::Result;
use std::{fs,path::{Path,PathBuf}};
pub fn detect_format(p:&Path)->String{match p.extension().and_then(|x|x.to_str()).unwrap_or("").to_ascii_lowercase().as_str(){
"gguf"=>"GGUF","safetensors"=>"SafeTensors","onnx"=>"ONNX","bin"=>"Binary","pt"|"pth"=>"PyTorch","mlx"=>"MLX",_=>"Directory/Unknown"}.into()}
pub fn model_name(p:&Path)->String{p.file_stem().or_else(||p.file_name()).and_then(|x|x.to_str()).unwrap_or("model").into()}
pub fn insert(s:&str)->Result<(String,String)>{let p=PathBuf::from(s);if !p.exists(){anyhow::bail!("Path does not exist: {s}")}Ok((model_name(&p),detect_format(&p)))}
pub fn scan()->Vec<(String,String,String)>{let mut o=Vec::new();let mut roots=Vec::new();if let Some(h)=dirs::home_dir(){roots.push(h.join("models"));roots.push(h.join(".cache/huggingface/hub"));}for r in roots{if let Ok(es)=fs::read_dir(r){for e in es.flatten(){let p=e.path();o.push((model_name(&p),p.display().to_string(),detect_format(&p)));}}}o}
