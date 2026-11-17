#!/usr/bin/env bash
# Arranca el servidor de desarrollo con recarga en caliente.
set -euo pipefail
cd "$(dirname "$0")/.."

printf "\033[1;35m▶\033[0m Leptos en \033[1;36mhttp://localhost:3000\033[0m (Ctrl+C para detener)\n\n"
exec trunk serve
