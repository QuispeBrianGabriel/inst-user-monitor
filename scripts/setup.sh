#!/usr/bin/env bash
# Prepara el repo recién clonado. Se ejecuta en el HOST, antes de abrir el Dev Container.
set -euo pipefail
cd "$(dirname "$0")/.."

ok()   { printf "\033[1;32m✔\033[0m %s\n" "$1"; }
fail() { printf "\033[1;31m✘\033[0m %s\n" "$1"; exit 1; }

command -v docker >/dev/null || fail "Docker no está instalado"
docker compose version >/dev/null 2>&1 || fail "Falta el plugin docker compose"
ok "Docker disponible"

if [ -f docker/.env ]; then
    ok "docker/.env ya existe (no se toca)"
else
    sed -e "s/^USER_UID=.*/USER_UID=$(id -u)/" \
        -e "s/^USER_GID=.*/USER_GID=$(id -g)/" \
        docker/.env.example > docker/.env
    ok "docker/.env creado con UID=$(id -u) GID=$(id -g)"
fi

docker volume inspect claude-config >/dev/null 2>&1 || docker volume create claude-config >/dev/null
ok "Volumen claude-config listo (sesión de Claude Code compartida entre contenedores)"

printf "\nSiguiente paso: abre la carpeta en VS Code → \033[1;36mDev Containers: Reopen in Container\033[0m\n"
