// Configuración de runtime para desarrollo (`trunk serve`).
// En producción el contenedor sobrescribe este fichero al arrancar a partir
// de las variables de entorno (ver docker/dockerfile.prod).
window.__APP_CONFIG__ = {
  apiUrl: "http://localhost:8080",
};
