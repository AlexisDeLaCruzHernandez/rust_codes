# Rust

Códigos de Rust

## Uso de Cargo

### Crear proyecto

```bash
$ cargo new <nombre_proyecto>
# Ejemplo
$ cargo new hello_cargo
```

Crea la carpeta `<nombre_proyecto>`. Dentro se encuentra el archivo `Cargo.toml`, que contiene la configuración de Cargo (incluyendo propiedades del programa y dependencias). También genera el archivo `src/main.rs` donde se escribe el código fuente.

### Compilar proyecto

```bash
$ cargo build
```

Compila el código creando un ejecutable por defecto en `target/debug/<nombre_proyecto>` y un archivo `Cargo.lock` que hace un seguimiento de las versiones exactas de las dependencias.

```bash
$ cargo build --release
```

Compila el código con optimizaciones para que el programa corra más rápido. Como demora más tiempo en procesarse, se suele usar solo para versiones finales. Genera el ejecutable en `target/release/<nombre_proyecto>`.

```bash
$ cargo check
```

Verifica si el código compila correctamente pero sin generar un ejecutable, por lo que es más rápido que hacer un `cargo build` convencional.

### Compilar y ejecutar proyecto

```bash
$ cargo run
```

Compila el código (igual que `cargo build`) y ejecuta el programa.

### Ver documentación

```bash
$ cargo doc --open
```

Abre la documentación en una página web, nos permite visualizar también las dependencias instaladas.
