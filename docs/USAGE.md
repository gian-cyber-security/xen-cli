# XEN CLI Usage

## Local model workflow

1. Put the model on the local machine.
2. Run `/insert <path>`.
3. Run `/models`.
4. Select it with `/use <model>`.

`/load <path>` can register a path for the current configuration.

## Permissions

Use `/settings-xen` to inspect permissions. The three modes are ON, ASK, and OFF.

Sensitive capabilities such as network access, project editing, shell access, and MCP remain configurable.

## Hugging Face

Run `/add-hf-api <token>` before using the HF API commands. XEN CLI does not silently use an HF credential.

## MCP

Run `/mcp add`, paste one JSON object, then inspect it with `/mcp list` and `/mcp info <name>`.

HTTP MCP requires Network Access. Local stdio MCP requires permission to start the local process.

## IDE

`/ide` checks for supported coding environments. The CLI itself does not require an IDE for general model or tool workflows.
