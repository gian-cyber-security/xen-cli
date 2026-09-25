use anyhow::{Context, Result};
use crate::{config, runtime};

pub fn chat(cfg: &config::Config, prompt: &str) -> Result<()> {
    let name = cfg.active_model.as_deref().context("No active model. Use /use <model>.")?;
    let model = cfg.models.iter().find(|m| m.name == name).context("Active model is not registered.")?;
    match model.format.as_str() {
        "GGUF" => {
            let output = runtime::execute("llama-cli", &["-m", &model.path, "-p", prompt, "-n", "256"])?;
            print!("{output}");
        }
        "Directory/Unknown" | "SafeTensors" | "PyTorch" | "ONNX" | "MLX" | "Binary" => {
            println!("No built-in native backend is registered for {} yet.", model.format);
            println!("Model metadata is recognized; connect a compatible local inference server/runtime for chat.");
        }
        _ => println!("Unsupported model format: {}", model.format),
    }
    Ok(())
}
