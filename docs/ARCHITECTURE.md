# XEN CLI Architecture

The CLI core is written in Rust for a portable native executable. Model inference is intentionally backend-based so XEN CLI can support different runtimes and formats without forcing every model into one framework.

Planned layers:

- CLI and terminal UI
- Agent engine
- Model registry and inference backends
- Hugging Face integration
- Local model loader
- Tool runtime
- Permission manager
- MCP manager
- Project/IDE integration

Supported targets: Windows, macOS, and Linux.

Model files are external runtime assets and are never committed to the repository.
