class: center, middle

<img src="../assets/images/rustmx-logo.svg" alt="RustMX" width="250rem" height="auto">

# Sesión 1: Conceptos básicos, tipos de datos, estructuras de control

---

# Instalación de Rust y requisitos

La forma recomendada para instalar *Rust* es hacerlo mediante _rustup_.

>>>```curl https://sh.rustup.rs -sSf | sh```

_rustup_ es la herramienta para manejar Rust y el proceso antes realizado instala
también _rustc_ y _cargo_.

Cabe señalar que el método antes mencionado es solo para sistemas *Nix. Si tú
tienes Windows debes seguir las instrucciones listadas en la siguiente página:
https://forge.rust-lang.org/other-installation-methods.html

Esta sesion asume que se ha instalado Rust, Cargo y otras herramientas mencionadas en el [Get-Started](https://www.rust-lang.org/learn/get-started) de la pagina oficial de Rust.

---

## Conceptos basicos

### Compilar y ejecutar
El codigo suele estar dentro de archivos `.rs` y similar a C o C++, se usa:
1. Un comando para compilar `rustc file.rs` genera un ejecutable `file` _(o file.exe en windows)_
2. Resta solo ejecutarlo `./file` _(o file.exe en windows)_

### Cargo
Como un repaso vale la pena mencionar que **Cargo** es una herramienta construir el projecto en Rust y administrador de paquetes.  Por tanto para:

- Generar un proyecto nuevo `cargo new {nombre-proyecto}` 
```
nombre-proyecto
|- Cargo.toml <- el manifiesto donde se indica metadata y dependencias [(leer mas)](https://github.com/toml-lang/toml)
|- src
  |- main.rs  <- el codigo
```
- Compilar mi proyecto `cargo build` _(se genera un Cargo.lock para definir las versiones exactas de las dependencias usadas)_
- Checar errores sin compilar `cargo check`
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

### Variables
Las variables son referencias a los datos en memoria.  Se usa `let` para su creacion.  Las variables no mutan por defecto, para cambiar esto se usa `mut`.
```
let foo = 5; // no mutable
let mut bar = String::new(); // mutable
```

La sintaxis `::new` indica que `new` es una funcion asociada al tipo de dato `String` que se define en el tipo de dato y esta disponible para todas las instancias de este.  En este caso, `new` es una funcion comun en muchos tipos de dato usada para crear nuevos valores. 

### Result
Muchas funciones de I/O devuelven un tipo de dato (enum) [Result](https://doc.rust-lang.org/nightly/std/result/enum.Result.html) que se usa para retornar y propagar errores.  Sus variantes son Ok para cuando la operacion es exitosa y Err representando un error y su valor.

`Result` es una buena solucion para recuperarse de errores que usan la macro [panic!](https://doc.rust-lang.org/std/macro.panic.html).

---

## Tipos de datos

---

## Estructuras de control

---


_Puedes acceder a mas documentos directamente en tu local ejecutando `rustup doc`_