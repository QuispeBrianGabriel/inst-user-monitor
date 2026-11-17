#!/usr/bin/env bash
# Genera un componente o una página de Leptos y lo registra en su mod.rs.
# Uso: scripts/generate.sh component|page nombre_en_snake_case
set -euo pipefail
cd "$(dirname "$0")/.."

kind=${1:-}
name=${2:-}
fail() { printf "\033[1;31m✘\033[0m %s\n" "$1"; exit 1; }

case "$kind" in
    component) dir=src/components ;;
    page)      dir=src/pages ;;
    *)         fail "Tipo desconocido: '$kind' (usa component o page)" ;;
esac

[ -n "$name" ] || fail "Falta el nombre. Ej: make $kind NAME=user_card"
[[ "$name" =~ ^[a-z][a-z0-9_]*$ ]] || fail "El nombre debe ir en snake_case: '$name'"

file="$dir/$name.rs"
[ -e "$file" ] && fail "Ya existe $file"

# snake_case → PascalCase
pascal=$(sed -E 's/(^|_)([a-z0-9])/\U\2/g' <<<"$name")

if [ "$kind" = page ]; then
    body="        <section class=\"page\">
            <h1>\"$pascal\"</h1>
        </section>"
else
    body="        <div class=\"$name\">\"$pascal\"</div>"
fi

cat > "$file" <<RS
use leptos::prelude::*;

#[component]
pub fn $pascal() -> impl IntoView {
    view! {
$body
    }
}
RS

echo "pub mod $name;" >> "$dir/mod.rs"
printf "\033[1;32m✔\033[0m Creado %s (%s) y registrado en %s/mod.rs\n" "$file" "$pascal" "$dir"

if [ "$kind" = page ]; then
    printf "  Añade la ruta en src/app.rs:\n"
    printf "    \033[2muse crate::pages::%s::%s;\033[0m\n" "$name" "$pascal"
    printf "    \033[2m<Route path=path!(\"/%s\") view=%s />\033[0m\n" "${name//_/-}" "$pascal"
fi
