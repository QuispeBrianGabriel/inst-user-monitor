#!/usr/bin/env bash
# Publica una versión: actualiza Cargo.toml, crea el commit y el tag vX.Y.Z y los sube.
# El tag dispara .github/workflows/release.yml (imagen en GHCR + GitHub Release).
# Uso: scripts/release.sh 0.1.0
set -euo pipefail
cd "$(dirname "$0")/.."

fail() { printf "\033[1;31m✘\033[0m %s\n" "$1"; exit 1; }

version=${1:-}
[ -n "$version" ] || fail "Falta la versión. Ej: make release V=0.1.0"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "Versión no válida: '$version' (formato X.Y.Z)"
tag="v$version"

[ "$(git branch --show-current)" = main ] || fail "Las versiones se publican desde main"
[ -z "$(git status --porcelain)" ] || fail "Hay cambios sin commitear"
git fetch --tags --quiet origin
[ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ] || fail "main no está sincronizada con origin/main"
git rev-parse -q --verify "refs/tags/$tag" >/dev/null && fail "El tag $tag ya existe"

current=$(git describe --tags --abbrev=0 2>/dev/null || echo "ninguna")
printf "Versión actual: \033[2m%s\033[0m → nueva: \033[1;36m%s\033[0m\n" "$current" "$tag"
read -rp "¿Publicar $tag? [s/N] " answer
[[ "$answer" =~ ^[sS]$ ]] || { echo "Cancelado"; exit 0; }

sed -i -E "0,/^version = \".*\"/s//version = \"$version\"/" Cargo.toml
cargo update --workspace --quiet

git add Cargo.toml Cargo.lock
git commit -m "chore(release): $tag"
git tag -a "$tag" -m "Release $tag"
git push origin main "$tag"

printf "\033[1;32m✔\033[0m %s publicado. Sigue el progreso en GitHub → Actions → Release\n" "$tag"
