# front

Frontend en [Leptos](https://leptos.dev) 0.8 (CSR) servido con [Trunk](https://trunkrs.dev).

## Desarrollo (devcontainer)

Todo corre dentro de Docker; en el host solo hace falta Docker y VS Code con la extensión *Dev Containers*.

1. En el host, una sola vez: `make setup` (crea `docker/.env` con tu UID/GID y el volumen de Claude Code).
2. En VS Code: **Dev Containers: Reopen in Container**.
3. Dentro del contenedor, `make` muestra todas las tareas. Las principales:

| Tarea | Qué hace |
| --- | --- |
| `make run` | `trunk serve` en http://localhost:3000 |
| `make check` | formato, clippy, tests y build: lo mismo que el pipeline |
| `make format` | aplica `cargo fmt` + `leptosfmt` |
| `make component NAME=user_card` | crea `src/components/user_card.rs` y lo registra |
| `make page NAME=user_detail` | crea `src/pages/user_detail.rs` y lo registra |
| `make build` | build optimizado en `dist/` |

## Producción

```sh
docker compose -f docker/compose.yml --profile prod up -d --build web-prod   # http://localhost:8080
```

Build multi-stage: compila con Trunk y sirve `dist/` con nginx (con fallback SPA a `index.html`).

La URL de la API se fija **al compilar** con la variable `API_URL` (por defecto `http://localhost:8080`), p. ej. `docker build --build-arg API_URL=https://api.example.com ...`. La API debe permitir el origen del front en `CORS_ORIGIN`.

## CI / Release

- `.github/workflows/pipeline.yml`: en cada push ejecuta `make check`.
- `.github/workflows/release.yml`: con un tag `v*` espera a que el pipeline de ese commit pase, publica la imagen en `ghcr.io/<owner>/<repo>:X.Y.Z` y crea la GitHub Release con el changelog generado por git-cliff (`cliff.toml`, Conventional Commits).
- `make release V=0.1.0` (desde `main`, sin cambios pendientes) actualiza `Cargo.toml`, crea el commit `chore(release): v0.1.0` y el tag, y los sube.

## Estructura

```
src/
  main.rs          # punto de entrada (monta <App/>)
  app.rs           # estado global y rutas
  config.rs        # nombre de la app, URL de la API, rutas y claves de localStorage
  models/          # tipos de dominio (User, Session, peticiones, errores)
  services/        # acceso a API y localStorage
  state/           # estado global vía contexto (AuthState: use_auth())
  utils/           # funciones puras y testeadas (validación)
  components/
    ui/            # piezas genéricas: TextField, SubmitButton, Alert, ...
    auth/          # LoginForm, RegisterForm
    ...            # Header, ProtectedLayout (guarda de rutas privadas)
  pages/           # una vista por ruta (AuthPage, Dashboard, NotFound)
style/
  main.css         # tokens de diseño, reset y layout
  components.css   # estilos de components/ui
  auth.css         # página de acceso
public/            # assets estáticos copiados a la raíz de dist/ (icon.svg, manifest)
scripts/           # lógica de las tareas del Makefile
docker/            # dockerfile.dev, dockerfile.prod, compose.yml, .env.example
.devcontainer/     # configuración de VS Code Dev Containers
```

Para añadir una página: crea `src/pages/<nombre>.rs`, regístrala en `src/pages/mod.rs` y añade un `<Route>` en `src/app.rs`.

### Rutas

| Ruta | Vista |
| --- | --- |
| `/` | Login (`/?mode=register` → registro). Con sesión, redirige a `/dashboard` |
| `/dashboard` | Privada (`ProtectedLayout`): sin sesión, redirige a `/` |

> `services/auth_service.rs` habla con la API: `login` pide el token y luego `/auth/me`; `register` crea la cuenta e inicia sesión enseguida, así que el usuario entra directo a `/dashboard`.

### Icono

El favicon es el emoji 🛰️ en `public/icon.svg` (SVG, sin imágenes rasterizadas); también lo usa `public/site.webmanifest`.
