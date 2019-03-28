<img src="../assets/images/rustmx-logo.svg" alt="RustMX" width="250rem" height="auto">

# Sesión 4: _Collections_

---

# Agenda

1. Introducción
2. String
3. Vec\<T\>
4. HashMap\<K,V\>

---

# Introducción

La biblioteca estándar de Rust incluye una serie de estructuras de datos muy útiles llamadas colecciones. En esta sección del curso vamos a conocerlas y a practicar un poco con ellas.

# String

> "Es útil tratar las String en el contexto de las colecciones porque se implementan como **una colección de bytes**, además de algunos métodos para proporcionar funcionalidad útil cuando esos bytes se interpretan como texto."
> -- <cite>[El Libro](https://doc.rust-lang.org/book/ch08-02-strings.html)</cite>

[Ejemplo 1.1: Creación de String:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d36f42f3ea472f1f9452a11323bfe82a)
```rust
fn main() {
    let s = String::from("String creada en la declaración");
    println!("{}", s);
    
    let s = "String declarada desde una literal con to_string()".to_string();
    println!("{}", s);
    
    let mut s = String::new();
    s.push_str("String inicializada mutable, actualizada después con push_str()");
    println!("{}", s);
}
```
```shell
String creada en la declaración
String declarada desde una literal con to_string()
String inicializada mutable, actualizada después con push_str()
```

---

[Ejemplo 1.2: push_str() no se apropia de las variables:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6858e04f0980b3b86757b055df281f27)
```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
```
```shell
s2 is bar
```
---

[Ejemplo 1.3: push() agrega un único carácter:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=874182da384de8be78822bc245be3af7)
```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}
```
```shell
lol
```

---

[Ejemplo 1.4: Concatenación de Strings:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=951abff3786d6dcd904c2bf8c96cc19a)
```rust
fn main() {
    let s1 = String::from("¡Hola, ");
    let s2 = String::from("mundo!");
    let s3 = s1 + &s2; // nota: s1 fue movida aquí y ya no puede ser usada
    println!("{}", s3);
}
```
```shell
¡Hola, mundo!
```

---

[Ejemplo 1.5: Concatenaciones más complicadas con la macro format!:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=df6badfdc9f1d4aa3bd0bdbbaf03eabc)
```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    
    println!("{}", s);
}
```
```shell
tic-tac-toe
```

---


# Vec\<T\>

El vector permite almacenar más de un valor en una estructura de datos. Cada ítem se coloca uno junto a otro en la memoria.

_Un vector solamente almacena valores del mismo tipo_.

## Ejemplos

[Ejemplo 2.1: Declaración de un vector i32 mutable y el método push para agregar valores:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8894b75e283e21289bd56b53e62f25f8)
```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    
    v.push(1);
    v.push(2);
    v.push(3);
    
    println!("El primer ítem del vector v es {}", v[0]);
    println!("El vector v contiene estos valores: {:?}", v);
}
```
---
[Ejemplo 2.2. Declaración de un vector inmutable, incluyendo los valores y dejando que Rust infiera el tipo de dato:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=75f072fb35a6b7c32f973391c90e28fc)
```rust
let v = vec![1, 2, 3];
``` 
---
[Ejemplo 2.3. ¿Quieres llevar un ítem del vector a otra variable?:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=db8c124958c22a1247fd7749f55d585e)
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    let tercero: &i32 = &v[2];
    println!("El tercer elemento contiene {}", tercero);
}
```

[Ejemplo 2.4. La memoria ocupada por el vector es liberada tan pronto sale de alcance:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4a814d7630e367a7def02f539003f4b9)
```rust
fn main() {

    {
        let v = vec![1, 2, 3, 4, 5];
    }

    println!("{:?}", v);
} 
```
```shell
    println!("{:?}", v);
|                    ^ not found in this scope
```

---

[Ejemplo 2.5. Leyendo elementos de un vector:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e9389f00e8b7096c657b17b134e3a8b1)
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let tercero: &i32 = &v[2];
    println!("El tercer elemento es {}", tercero);

    match v.get(2) {
        Some(tercero) => println!("El tercer elemento es {}", tercero),
        None => println!("No es el tercer elemento."),
    }
}
```
```shell
El tercer elemento es 3
El tercer elemento es 3
```

---

[Ejemplo 2.6. Iteración:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4e601bedfc96b5245ea9674f98986aff)
```rust
fn main() {
    let v = vec![100, 32, 57];
    
    for i in &v {
        println!("{}", i);
    }
}
```
```shell
100
32
57
```

---

[Ejemplo 2.7. Iteración y operación con elementos mutables:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cb70086c12d8a73acebfe69206527e73)
```rust
fn main() {
    let mut v = vec![100, 32, 57];
    
    for i in &mut v {
        // * es el operador dereference, para obtener el valor antes de cambiarlo
        *i += 50;
        println!("{}", i);
    }
}
```
```shell
150
82
107
```

---

# HashMap\<K,V\>

Este tipo de datos almacena valores identificados por una clave.

En otros lenguajes de programación podemos encontrar este tipo de dato con diferentes nombres: _hash, hash table, dictionary, associative array, map, object_.

---

[Ejemplo 3.1. Creación de un nuevo Hash Map](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cf726b7d2e3c69a6b503cc9caf7ea829)
```rust
use std::collections::HashMap;

fn main() {
    let mut valoraciones = HashMap::new();
    valoraciones.insert(String::from("Us"), 95);
    valoraciones.insert(String::from("Captain Marvel"), 78);
    valoraciones.insert(String::from("Gloria Bell"), 94);
}
```

El _Hash Map_ es el menos usado de los elementos comunes para colecciones, así que no se carga automáticamente durante el _prelude_, por lo que debemos invocarlo explícitamente con _use std::collections::HashMap;_.

---

[Ejemplo 3.2. Iteración de un Hash Map](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=11a12f1423c26fa04e50506fb84ef07b)
```rust
use std::collections::HashMap;

fn main() {
    let mut valoraciones = HashMap::new();
    valoraciones.insert(String::from("Us"), 95);
    valoraciones.insert(String::from("Captain Marvel"), 78);
    valoraciones.insert(String::from("Gloria Bell"), 94);

    for (clave, valor) in &valoraciones {
        println!("{}: {}%", clave, valor);
    }
}
```
```shell
Captain Marvel: 78%
Gloria Bell: 94%
Us: 95%
```

---

Nota: En los Hash Maps, al igual que con los vectores, se requiere que los valores sean del mismo tipo:

```shell
  |
6 |     valoraciones.insert(String::from("Captain Marvel"), 78.7);
  |                                                         ^^^^ expected integer, found floating-point number
  |
  = note: expected type `{integer}`
             found type `{float}`
```

---

[Ejemplo 3.3. Obtener el valor de un Hash Map usando la clave](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=85bd93cd908c2e22cb46e466f2bd2b8e)
```rust
use std::collections::HashMap;

fn main() {
    let mut valoraciones = HashMap::new();
    valoraciones.insert(String::from("Us"), 95);
    valoraciones.insert(String::from("Captain Marvel"), 78);
    valoraciones.insert(String::from("Gloria Bell"), 94);
    
    let la_clave = String::from("Gloria Bell");
    let el_valor = valoraciones.get(&la_clave);
    
    match el_valor {
        Some(x) => println!("La valoración de {} es {}%.", la_clave, x),
        None => println!("No tenemos una valoración para {}.", la_clave),
    }
}
```
```shell
La valoración de Gloria Bell es 94%.
```