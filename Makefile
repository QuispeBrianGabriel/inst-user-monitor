# =============================================================================
#  Inst User Monitor Front — interfaz del desarrollador
#
#  Ejecuta `make` o `make help` para ver todas las tareas.
#  La lógica vive en scripts/: aquí solo se llaman comandos.
#
#  Todas las tareas se ejecutan dentro del Dev Container, salvo `setup`
#  (en el host, antes de abrirlo por primera vez) y `stack*` (en el host,
#  porque necesitan Docker).
#
#  Al añadir una tarea, añade también su línea en `help`.
# =============================================================================

# bash y no sh (dash): con un solo comando, bash se reemplaza por él. dash se
# queda en medio y, tras un Ctrl+C, se mata con SIGINT aunque el comando haya
# terminado bien, y make imprime "*** Interrupt" (ver scripts/graceful.sh).
SHELL         := bash
.DEFAULT_GOAL := help
MAKEFLAGS     += --no-print-directory

# Envoltorio de cierre ordenado con Ctrl+C (ver scripts/graceful.sh)
GRACEFUL := scripts/graceful.sh
CARGO    := $(GRACEFUL) "🛑 Tarea de cargo interrumpida" cargo
TRUNK    := $(GRACEFUL) "🛑 Tarea de trunk interrumpida" trunk

# Parámetros opcionales (se pasan como `make tarea VAR=valor`)
T    ?=
NAME ?=
V    ?=

# Repo del back (hermano de este): ahí vive el stack completo (docker/stack.yml)
BACK_DIR ?= $(patsubst %/scripts/stack.sh,%,$(firstword $(wildcard ../back/scripts/stack.sh ../inst-user-monitor-back/scripts/stack.sh)))

.PHONY: setup help tree \
        stack stack-down stack-logs stack-reset \
        install run \
        check test format format-check \
        build clean \
        component page \
        version release

# =============================================================================
#  📚 Ayuda
# =============================================================================

# Formatos de printf usados por `help`:
#   TITLE → título de grupo
#   ITEM  → tarea + descripción
#   ARGS  → tarea + descripción + parámetros (atenuados)
TITLE := \n\033[1;33m%s\033[0m\n
ITEM  := "  \033[1;36m%-15s\033[0m %s\n"
ARGS  := "  \033[1;36m%-15s\033[0m %s \033[2m%s\033[0m\n"

help:
	@printf "\n\033[1;35m  📡  Inst User Monitor Front\033[0m \033[2m— make <tarea> [VAR=valor]\033[0m\n"

	@printf "$(TITLE)" "🖥️  Host — se ejecuta FUERA del Dev Container"
	@printf $(ITEM) "setup"         "🧰 Prepara el repo recién clonado (una sola vez)"
	@printf $(ARGS) "stack"         "🧩 Front + API + Postgres en local, como en Railway" "[BACK_DIR=]"
	@printf $(ITEM) "stack-logs"    "📜 Sigue los logs del stack"
	@printf $(ITEM) "stack-down"    "⏹️  Detiene el stack (conserva la base)"
	@printf $(ITEM) "stack-reset"   "💣 Detiene el stack y borra su base (pide confirmar)"

	@printf "$(TITLE)" "📚 General"
	@printf $(ITEM) "help"          "📖 Muestra esta ayuda"
	@printf $(ITEM) "tree"          "🌳 Árbol del código del proyecto"

	@printf "$(TITLE)" "🚀 Aplicación"
	@printf $(ITEM) "install"       "📥 Descarga las dependencias (cargo fetch)"
	@printf $(ITEM) "run"           "▶️  Arranca la app en :3000 (trunk serve)"

	@printf "$(TITLE)" "🧪 Calidad"
	@printf $(ITEM) "check"         "✅ Lo mismo que el pipeline"
	@printf $(ARGS) "test"          "🧪 Ejecuta los tests" "[T=nombre_del_test]"
	@printf $(ITEM) "format"        "🎨 Corrige el formato"
	@printf $(ITEM) "format-check"  "🔍 Verifica el formato sin tocar nada"

	@printf "$(TITLE)" "🏗️  Build"
	@printf $(ITEM) "build"         "📦 Genera la app optimizada en dist/"
	@printf $(ITEM) "clean"         "🧹 Borra dist/ y target/"

	@printf "$(TITLE)" "🧩 Generadores"
	@printf $(ARGS) "component"     "🧱 Crea un componente" "NAME=user_card"
	@printf $(ARGS) "page"          "📄 Crea una página" "NAME=user_detail"

	@printf "$(TITLE)" "🏷️  Release"
	@printf $(ITEM) "version"       "🔖 Última versión publicada"
	@printf $(ARGS) "release"       "🚢 Publica la imagen (pide confirmar)" "V=0.1.0"

	@printf "\n\033[2m  Docs: README.md\033[0m\n\n"

.DEFAULT:
	@printf "\033[1;31m✘\033[0m La tarea '%s' no existe. Usa \033[1;36mmake help\033[0m para ver todas.\n" "$@"

# Se ejecutan en el host: necesitan Docker
setup:
	@scripts/setup.sh

# El stack lo define el back; se le pasa este repo como front
stack stack-down stack-logs stack-reset:
	@if [ ! -x "$(BACK_DIR)/scripts/stack.sh" ]; then \
		printf "\033[1;31m✘\033[0m No encuentro el repo del back$(if $(BACK_DIR), en $(BACK_DIR)). Indícalo: make $@ BACK_DIR=/ruta/al/back\n"; \
	else \
		$(MAKE) -C "$(BACK_DIR)" $@ FRONT_DIR="$(CURDIR)"; \
	fi

tree:
	@$(GRACEFUL) "🛑 Cancelado" tree -a --dirsfirst -I '.git|target|dist|scripts|docker|.devcontainer|.github'

# =============================================================================
#  🚀 Aplicación
# =============================================================================

install:
	@$(CARGO) fetch

run:
	@$(GRACEFUL) "⏹️  App detenida correctamente" scripts/serve.sh

# =============================================================================
#  🧪 Calidad
# =============================================================================

check:
	@$(GRACEFUL) "🛑 Comprobaciones interrumpidas" scripts/check.sh

# Con T se filtran los tests por nombre: make test T=header
test:
	@$(CARGO) test $(if $(T),'$(T)')

format:
	@$(CARGO) fmt && leptosfmt --quiet src

format-check:
	@$(CARGO) fmt --check && leptosfmt --check src

# =============================================================================
#  🏗️ Build
# =============================================================================

build:
	@$(TRUNK) build --release

clean:
	@$(CARGO) clean && rm -rf dist

# =============================================================================
#  🧩 Generadores
# =============================================================================

component:
	@scripts/generate.sh component '$(NAME)'

page:
	@scripts/generate.sh page '$(NAME)'

# =============================================================================
#  🏷️ Release
# =============================================================================

version:
	@git describe --tags --abbrev=0 2>/dev/null || printf "\033[2mAún no hay ninguna versión publicada\033[0m\n"

release:
	@scripts/release.sh "$(V)"
