# XEN CLI

XEN CLI is a cross-platform, local-first AI agent CLI for working with XEN models, other local AI models, Hugging Face models, tools, projects, and MCP servers.

## Highlights

- Local XEN and third-party model loading
- Hugging Face model integration
- MCP support for local stdio servers
- Configurable permissions with ON / ASK / OFF
- Cross-platform Windows, macOS, and Linux support
- Coding tools can integrate with supported IDEs
- Web, filesystem, shell, project, voice, and vision capabilities are permission-controlled

## Quick Start

Build from source:

```bash
cargo build --release
```

Run:

```bash
cargo run
```

## Hugging Face

Before using Hugging Face models through the API, configure access with:

```text
/add-hf-api
```

Local models do not require an HF API key.

## Local Models

Register a local model:

```text
/insert <local-model-path>
```

Or load one temporarily:

```text
/load <path>
```

Then select it:

```text
/use <model>
```

XEN CLI is designed to detect supported model formats and architectures where possible. Large model files are intentionally not stored in this repository.

## Permissions

Use:

```text
/settings-xen
```

Permission modes:

- ON — allow automatically
- ASK — ask before use
- OFF — disable

Examples:

```text
/settings-xen network on
/settings-xen shell ask
/settings-xen files off
/settings-xen mcp ask
```

On Windows, the shell permission can be displayed as "Allow CMD". On macOS and Linux it can be displayed as "Allow Terminal". The internal setting is `shell`.

## MCP

```text
/mcp
/mcp add
/mcp list
/mcp info <name>
/mcp enable <name>
/mcp disable <name>
/mcp remove <name>
/mcp reload
```

For `/mcp add`, XEN CLI accepts an MCP JSON configuration, validates it, detects the transport, and registers the server. Secrets must not be printed in logs.

HTTP MCP servers require Network Access to be ON or ASK. Local stdio MCP servers require permission to start the local process.

## Commands

```text
/xen-cli
/help
/version
/status
/models
/model
/use <model>
/load <path>
/unload
/info <model>
/scan
/refresh
/project
/project <path>
/files
/read <file>
/tools
/tools on
/tools off
/new
/history
/save <name>
/hf
/hf search <query>
/hf install <model>
/hf info <model>
/hf remove <model>
/add-hf-api
/voice
/vision
/config
/settings
/settings-xen
/diagnostics
/ide
/mcp
/exit
/uninstall-xen-cli
/insert <path>
```

Invalid commands should show a useful suggestion or usage message instead of raw error codes or stack traces.

## Safety and Privacy

XEN CLI is local-first. Local model files stay on the user's machine unless the user explicitly configures an external service. Tool access is controlled by the permission settings.

## License

MIT License.
