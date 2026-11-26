# Política de seguridad

## Versiones con soporte

| Versión | Soporte |
|---|---|
| 0.1.x | Sí |

## Modelo de amenazas

Remote-RS controla un PC con Windows (multimedia, volumen, ratón, apagado, apertura de URL y una terminal PowerShell). Está pensado **solo para una red local de confianza**:

- Por defecto escucha únicamente en `127.0.0.1:3000`.
- La terminal remota está **desactivada** salvo que se defina `REMOTE_RS_TERMINAL_TOKEN`, y entonces exige la cabecera `X-Remote-Token`.
- El resto de la API (apagar, bloquear, ratón, volumen, abrir URL) **no tiene autenticación**: cualquiera que alcance el puerto puede usarla.
- Nunca exponga el puerto a Internet ni lo abra en redes públicas.

## Cómo reportar una vulnerabilidad

Abra un aviso privado desde **Security → Report a vulnerability** en el repositorio. No publique el detalle en un issue abierto.
