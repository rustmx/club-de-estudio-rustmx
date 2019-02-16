<img src="../assets/images/rustmx-logo.svg" alt="RustMX" width="250rem" height="auto">

# Sesión 1: Conceptos básicos, tipos de datos, estructuras de control

---

## Agenda

- [Instalación](#instalación)
- [Conceptos Basicos](#conceptos-basicos)
- [Tipos de datos](#tipos-de-datos)
- [Estructuras de control](#estructuras-de-control)
- [Ejercicios](ejercicios/lab-01)

---

## Instalación

La forma recomendada para instalar *Rust* es hacerlo mediante _rustup_.
```
curl https://sh.rustup.rs -sSf | sh
```

_rustup_ es la herramienta para manejar Rust y el proceso antes realizado instala
también _rustc_ y _cargo_.

Cabe señalar que el método antes mencionado es solo para sistemas *Nix. Si tú
tienes Windows debes seguir las instrucciones listadas en la siguiente página:
https://forge.rust-lang.org/other-installation-methods.html

Tambien es recomendable que sigas las instrucciones del [Get-Started](https://www.rust-lang.org/learn/get-started) en la pagina oficial para tener Rust, Cargo y otras herramientas utiles.

---
## Conceptos basicos

### Compilar y ejecutar
El codigo suele estar dentro de archivos `.rs` y similar a C o C++, se usa:
1. Un comando para compilar `rustc file.rs` genera un ejecutable `file` _(o file.exe en windows)_
2. Resta solo ejecutarlo `./file` _(o file.exe en windows)_

---
### Cargo
Como un repaso vale la pena mencionar que **Cargo** es una herramienta construir el projecto en Rust y administrador de paquetes.  Por tanto para:

- Generar un proyecto nuevo `cargo new {nombre-proyecto}` 
```
nombre-proyecto
|- Cargo.toml <- metadata y dependencias - https://github.com/toml-lang/toml
|- src
  |- main.rs  <- el codigo
```
- Compilar mi proyecto `cargo build` _(se genera un Cargo.lock para definir las versiones exactas de las dependencias usadas)_
- Checar errores sin compilar `cargo check`
- Compilar (si lo requiere) y ejecutar mi proyecto `cargo run` 
- Ejecutar las pruebas `cargo test` 
- Generar documentos `cargo doc` 
- Publicar una libreria en [crates.io](https://crates.io/) `cargo publish`

---
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

_Cuando se importan nuevos crates a un proyecto hay que compilarlo nuevamente._

---
### Variables
Las variables son referencias a los datos en memoria.  Se usa `let` para su creacion.  Las variables no mutan por defecto, para cambiar esto se usa `mut`.
```
let foo = 5; // no mutable
let mut bar = String::new(); // mutable
foo = 6; // error en tiempo de compilacion
```

La sintaxis `::new` indica que `new` es una funcion asociada al tipo de dato `String` que se define en el tipo de dato y esta disponible para todas las instancias de este.  En este caso, `new` es una funcion comun en muchos tipos de dato usada para crear nuevos valores.

La linea 3 en el codigo arriba causa un error en tiempo de compilacion por tratar de asignar un valor a una variable no mutable. De esta manera el compilador garantiza que cuando defines que un valor no va a cambiar, efectivamente no cambie.  Pero la mutabilidad es muy util cuando se opera con estructuras de datos grandes, ya que es mas rapido cambiar los valores en el mismo lugar que estar copiando toda la estructura y alojando en memoria nuevas instancias cada vez.

---
#### Shadowing
Existe una forma de declarar una variable nueva con el mismo nombre y usando el mismo valor que una previa.  De esta manera se dice que la nueva variable emerge de la sombra de la primera (shadows).
```
  let val = 4;
  let val = val + 5;
  println!("The value is {}", val);
```
[Intentalo](https://repl.it/@wdonet/rust-shadowing)

El programa primero enlaza `val` al valor `4` y luego emerge de su sombra al hacer `let val = val ...` tomando el valor original y agregando `5`.

Con `mut` el compilador sabe que una variable sera mutable en todo su ciclo de vida.  _Shadowing_ puede definir una serie de transformaciones sobre los valores de la variable pero al final de estas se mantiene como no mutable.

Otra ventaja sobre `mut` es que con _Shadowing_ el tipo de dato en dicha transformacion.
```
  let spaces = "   ";
  let spaces = spaces.len();
  println!("Total spaces {}", spaces);
  let mut number = "four";
  number = 5; // error en tiempo de compilacion
```
[Intentalo](https://repl.it/@wdonet/shadowing-mut)

---
### Constantes
A diferencia de las variables, las constantes _siempre son no mutables_ y deben ser anotadas con su tipo explicitamente.  Se definen solamente a traves de expresiones de constante como abajo y no como resultado de una llamada a una funcion, esto es, que no podrian ser definidas en tiempo de ejecucion.

```
const MAX_RESULTS_PER_REQUEST: u32 = 100_000;
```

La convencion para nombres de constantes es de usar solo mayusculas y subrayados entre palabras.  _El subrayado en numeros es para mejorar la legibilidad de estos pero no agrega ninguna funcionalidad._

Las constantes son validas por el tiempo en que se ejecuta el programa y dentro del alcance (scope) en el que fueron declaradas.

---
### Result
Muchas funciones de I/O devuelven un tipo de dato (enum) [Result](https://doc.rust-lang.org/nightly/std/result/enum.Result.html) que se usa para retornar y propagar errores.  Sus variantes son Ok para cuando la operacion es exitosa y Err representando un error y su valor.

`Result` es una buena solucion para recuperarse de errores que usan la macro [panic!](https://doc.rust-lang.org/std/macro.panic.html). **Panic** es el termino que Rust usa cuando un programa termina con error.

---
## Tipos de datos
Rust debe saber el tipo de datos de cada variable en tiempo de compilacion para saber como operar.  Sin embargo puede _inferir_ el tipo basado en el valor y en como se utiliza.

### Tipos Escalares
Representan un valor simple de 4 posibles: entero, punto flotante, logico y caracter.

---
#### Entero
Se define con una `i` (con signo +/-) o `u` (sin signo) seguido del numero de bits de espacio en memoria requeridos. Cada variante con signo puede almancenar numeros en el rango de `-2^(n-1)` a `2^(n-1) - 1` y las variantes sin signo en un rango de `0` a `2^(n) - 1`.

|    Lenght    |    Signed     | Unsigned |
| :----------: | :-----------: | :------: |
|    8-bit     |      i8       |    u8    |
|    16-bit    |      i16      |   u16    |
|    32-bit    | i32 (default) |   u32    |
|    64-bit    |      i64      |   u64    |
|   128-bit    |     i128      |   u128   |
| arquitectura |     isize     |  usize   |

Si quieres depender de la arquitectura de la computadora usa `isize` y `usize`.  Su tamanño es de de 64 bits en una arquitectura de de 64-bits por ejemplo.  Estos tipos se usan generalmente para indexar algun tipo de coleccion.

Si llegas al limite del rango de un tipo, por ejemplo una variable tipo `u8` con valor `255` y quieras cambiarlo a `256` - esto se llama **Integer Overflow** - el cual provoca que el compilador cause una señal de panico en modo debug.  De otro modo, Rust hace algo llamado **two's complement wrapping** que significa hacer el `256` en `0`, el `257` en `1` y asi sucesivamente.

---
#### Punto flotante
Solo existen dos siguiendo la misma nomenclatura que para enteros: `f32` y `f64` (default por ser mas preciso y casi de la misma velocidad).

> _Note:_ Las operaciones matematicas clasicas son suma (+), resta (-), multiplicacion (*), division (/) y residuo (%)

#### Logico
Se usa la palabra reservada `bool` y es de tamaño de un bit. Se puede especificar con las palabras `true` y `false` como valores.
```
let t = true;
let f: bool = false;
```

#### Caracter
Este es especificado con un caracter en comillas simples `'😻'`.  Representa un valor escalar Unicode por lo que puede representar mas caracteres del codigo ASCII como caracteres Chinos, Japoneses, Koreanos o emojis.  Su rango de valores va de `U+0000` a `U+D7FF` y `U+E000` a `U+10FFFF`.

---
### Tipos Compuestos
Rust cuenta con dos tipos de dato compuestos y primitivos que pueden agrupar multiples valores.

#### Tuplas
Son una forma de agrupar valores de tipos de dato distintos.  Son de tamaño fijo, se crean escribiendo una lista de valores separados por coma y entre parentesis.  Cada posicion tiene un tipo de dato.  Para acceder a los valores de una tupla se puede usar **pattern matching** a traves de `destructuring` o con el nombre de la variable seguido de `.` y el indice del elemento (comenzando en cero).
```
  let tupla = (500, 9.3, 'z', "cadena");
  println!("Tupla {:?}", tupla);
  let (entero, flotante, caracter, cadena) = tupla;
  println!("entero : {}", entero);
  println!("flotante : {}", flotante);
  println!("caracter : {}", caracter);
  println!("cadena : {}", cadena);

  let dos_enteros: (i32, u8) = (500, 1);
  println!("Dos enteros con indice : {} y {}", dos_enteros.0, dos_enteros.1);
```
[Intentalo](https://repl.it/@wdonet/rust-tuples)

---
#### Arreglos
En este caso todos los elementos deben ser del mismo tipo.  Son de tamaño fijo, sus valores tambien se separan por comas dentro de corchetes.  Son utiles cuando se quiere tener los datos en el `Stack` en lugar del `Heap`.  Un `vector` es una coleccion similar que puede crecer en tamaño.

Para indicar el tipo de un array se usan corchetes con dos elementos dentro, el primero es el tipo de dato para los elementos y separado por un `;` le sigue el total de elementos en el arreglo. Si esto no se indica, todo es inferido.

Para acceder a elementos especificos se usan `[]` con el indice del elemento. Indice comienza en cero.
```
  let a = ["second", "minute", "hour", "day", "month", "year"];
  println!("Arreglo a {:?}", a);
  let b:[u8; 3] = [10, 20, 30];
  println!("Elemento b[0] {}", b[0]);
  println!("Elemento b[1] {}", b[1]);
  println!("Elemento b[2] {}", b[2]);
```
[Intentalo](https://repl.it/@wdonet/rust-arrays)

---
## Estructuras de control
El objetivo de estas es decidir que codigo ejecutar y si hacerlo repetidamente mientras una serie de condiciones se cumplen.

### Expresion `if`
Si la condicion _(debe ser logica o de tipo `bool`)_ se cumple la condicion despues del `if`, ejecuta el bloque de codigo encerrado entre `{}`, de lo contrario ejecuta el bloque de codigo despues de `else`.

```
  let grados = 10;
  if grados > 30 {
    println!("Apagar estufa en 1 minuto!");
  } else if grados < 5 {
    println!("Comenzando");
  } else {
    println!("Temperatura {}", grados);
  }
```

---
Como `if` es una expresion, tambien se puede utilizar a la derecha de `let` como un valor.  _En este caso ambos valores deben ser del mismo tipo de dato._
```
  let estatus = if grados > 5 {
    "caliente"
  } else {
    "frio"
  };
  println!("Estado {}", estatus);
```
[Intentalo](https://repl.it/@wdonet/rust-control-if)


Pero usar multiples `else-if` puede hacer el codigo ilegible, para esos casos se puede usar `match`.

```
  let grados = 13;
  match grados {
        // Empatar un simple valor
        1 => println!("frio!"),
        // o varios valores
        2 | 3 | 5 | 7 => println!("tibio"),
        // o un rango
        13...19 => println!("caliente"),
        // En cualquier otro caso
        _ => println!("Temperatura {}", grados),
    }
```

---
### Ciclos
El proposito es ejecutar un bloque de codigo mas de una vez.

|                                  `loop`                                  |                        `while`                        |                                                              `for`                                                              |
| :----------------------------------------------------------------------: | :---------------------------------------------------: | :-----------------------------------------------------------------------------------------------------------------------------: |
| Ejecuta el codigo por siempre o hasta que se termine manualmente (`^C`). | Ejecuta el codigo siempre que la condicion se cumpla. | Usual para recorrer elementos de un tipo de dato compuesto como un arreglo o tupla.  Tambien se suele usar con rangos `(1..4)`. |

---

#### Ejemplos:
```
  /* Loop */
  let mut counter = 10;
  let status = loop {
    if counter == 0 {
      break "Terminado";
    }
    println!("Numero {}", counter);
    counter -= 1;
  };
  println!("=> {}", status);

  /* While */
  counter = counter + 5;
  while counter !=0 {
    println!("Numero {}", counter);
    counter -= 1;
  }
  println!("While terminado");

  /* For */
  let elements = [3, 2, 1];
  for element in elements.iter() {
    println!("Numero {}", element);
  }
  println!("For terminado");
```

---
Utilizando un [rango](https://doc.rust-lang.org/std/ops/struct.Range.html) `(1..4)` como en:
```
  for number in (1..7).rev() {
    println!("Numero {}", number);
  }
```
Los rangos incluyen el numero del inicio pero no el numero del tope final.

_**Note : ** Se puede usar `break` en cualquier ciclo para dar por terminada su ejecucion._

[Intentalo](https://repl.it/@wdonet/rust-control-loops)

---
_Puedes acceder a mas documentos directamente en tu local ejecutando `rustup doc`_

Ahora vamos al primer [laboratorio](ejercicios/lab-01)