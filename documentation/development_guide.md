# Guía de Desarrollo: Backend Modular (Remote-RS)

Este documento detalla la estructura del backend y cómo añadir nuevas funcionalidades.

## Estructura de Archivos
```text
src/
├── main.rs                 # Servidor Axum y orquestación de rutas.
├── build.rs                # Script de compilación para el shim de C++.
├── modules/
│   ├── mod.rs              # Registro de módulos.
│   ├── native.rs           # Declaraciones FFI (Enlace con C++).
│   ├── media/
│   │   ├── metadata.rs     # Extracción de info GSMTC (Rust).
│   │   └── controls.rs     # Simulación de teclas multimedia (Enigo).
│   ├── volume/
│   │   └── mod.rs          # Interfaz de volumen (Llama a C++).
│   ├── system/
│   │   ├── mod.rs          # Registro de acciones de sistema.
│   │   ├── lock.rs         # Bloqueo de sesión.
│   │   └── power.rs        # Apagado, reinicio y suspensión.
└── native/
    ├── windows_shim.h      # Cabecera de funciones nativas.
    └── windows_shim.cpp    # Implementación nativa en C++ (Win32/CoreAudio).
```

## Cómo añadir una función nativa (C++)
1.  Define la función en `src/native/windows_shim.h` dentro del bloque `extern "C"`.
2.  Impleméntala en `src/native/windows_shim.cpp` usando el SDK de Windows.
3.  Declara la función en `src/modules/native.rs` dentro del bloque `unsafe extern "C"`.
4.  Crea un módulo en Rust que envuelva la llamada `unsafe` para exponerla de forma segura al resto de la app.

## Dependencias Críticas
- **`windows` (crate)**: Acceso a APIs de Windows desde Rust.
- **`cc` (build-dep)**: Compilador de C++ integrado en el flujo de Cargo.
- **`enigo`**: Simulación de periféricos.
- **`axum` & `tokio`**: Servidor web asíncrono.
