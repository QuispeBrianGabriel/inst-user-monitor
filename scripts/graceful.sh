#!/usr/bin/env bash
# Ejecuta un comando y, si se interrumpe con Ctrl+C, muestra un mensaje y sale con 0
# para que make no imprima "*** Interrupt".
# Uso: scripts/graceful.sh "mensaje" comando [args...]
set -u
msg=$1
shift

# Con un trap (y no ignorando la señal) el hijo sigue recibiendo SIGINT normalmente;
# este script solo espera a que termine.
trap ':' INT
"$@"
status=$?

if [ "$status" -eq 130 ]; then
    printf "\n%s\n" "$msg"
    exit 0
fi
exit "$status"
