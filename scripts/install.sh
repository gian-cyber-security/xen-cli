#!/usr/bin/env bash
set -euo pipefail
cargo install --path .
echo "XEN CLI installed as: xen"
echo "Run: xen"
