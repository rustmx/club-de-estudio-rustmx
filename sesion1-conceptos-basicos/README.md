class: center, middle

<img src="../assets/images/rustmx-logo.svg" alt="RustMX" width="250rem" height="auto">

# Sesión 1: Conceptos básicos, tipos de datos, estructuras de control

---

## Requisitos

Esta sesion asume que se ha instalado Rust, Cargo y otras herramientas mencionadas en el [Get-Started](https://www.rust-lang.org/learn/get-started) de la pagina oficial de Rust.

---

## Conceptos basicos

### Cargo
Como un repaso vale la pena mencionar que **Cargo** es una herramienta construir el projecto en Rust y administrador de paquetes.  Por tanto para:

- Generar un proyecto nuevo `cargo new {nombre-proyecto}` 
```
nombre-proyecto
|- Cargo.toml <- el manifiesto donde se indica metadata y dependencias
|- src
  |- main.rs  <- el codigo
```
- Compilar mi proyecto `cargo build` _(se genera un Cargo.lock para definir las versiones exactas de las dependencias usadas)_
- Compilar (si lo requiere) y ejecutar mi proyecto `cargo run` 
- Ejecutar las pruebas `cargo test` 
- Generar documentos `cargo doc` 
- Publicar una libreria en [crates.io](https://crates.io/) `cargo publish`

### Dependencias
Las librerias se encuentras en [crates.io](https://crates.io/), el cual es el registro de paquetes para Rust.  Dichos paquetes se llaman **"crates"**.

Estos se agregan con nombre y version bajo la linea q dice `[dependencies]` en `Cargo.toml` de la forma:
```
[dependencies]
ferris-says = "0.1"
```
Luego para usar en el codigo con `use` seguido de `::` para indicar la funcion exportada por dicho crate.
```
use ferris_says::say;
```

_Cuando se importan nuevos crates a un proyecto hay que compilar el proyecto nuevamente._

### Result
Muchas funciones de I/O devuelven un tipo de dato (enum) [Result](https://doc.rust-lang.org/nightly/std/result/enum.Result.html) que se usa para retornar y propagar errores.  Sus variantes son Ok para cuando la operacion es exitosa y Err representando un error y su valor.

`Result` es una buena solucion para recuperarse de errores que usan la macro [panic!](https://doc.rust-lang.org/std/macro.panic.html).

---

## Tipos de datos

---

## Estructuras de control

---
