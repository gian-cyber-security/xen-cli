use crate::config;

pub fn run(cfg: &config::Config) {
    println!("XEN CLI diagnostics");
    println!("Version: 0.2.0");
    println!("Platform: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
    println!("Models: {}", cfg.models.len());
    println!("Active model: {}", cfg.active_model.as_deref().unwrap_or("none"));
    println!("HF API: {}", if cfg.hf_api_key.is_some() {"configured"} else {"not configured"});
    println!("Config path: {}", config::path().map(|p| p.display().to_string()).unwrap_or_else(|_| "unavailable".into()));
}
