class: center, middle

<img src="../assets/images/rustmx-logo.svg" alt="RustMX" width="250rem" height="auto">

# Sesión 2: _Ownership_

---
class: center, middle

# Opciones de manejo de la memoria

---
class: center, middle

## A. Tu limpias tu propia memoria

* Lenguajes de bajo nivel como C, C++
* Se te puede olvidar
* propenso a crear punteros nulos

---
class: center, middle

## B. Colector de basura

* Lenguajes de alto nivel (python, java, ruby, javascript, php, ...)
* Sacrificas un poco de rendimiento
* ¿Cómo sabes si un valor ya se puede limpiar de la memoria?

---
class: center, middle

## C. Usas rust

* Cada valor en memoria tiene un único _dueño_ (una variable)
* La propiedad se va pasando al reasignar
* Las referencias apuntan a valores siempre válidos (verificado en tiempo de compilación)
* la memoria se libera cuando el dueño sale de scope

---
class: middle

## Como funciona en python

```python
a = [1, 2, 3, 4]
b = a
a.push(5)
print(b)
# [1, 2, 3, 4, 5]
```

## Como funciona en rust

```rust
fn main() {
    let mut a = vec![1, 2, 3, 4];
    let b = a;
    a.push(5);
    println!("{:?}", b);
    // No compila
}
```

Se dice que el valor se _movió_

---
class: middle

# Excepción de la regla

Si un tipo de dato implementa el trait Copy su semántica cambia:

```rust
fn main() {
    let mut a = 5; // los tipos de dato primitivos implementan copy
    let b = a;
    a += 1;
    println!("{:?} {:?}", a, b);
    // 6 5;
}
```

Nuevo espacio en memoria es reservado para el valor copiado

---
class: middle

# Funciones

En el paso de valores a funciones aplican las mismas reglas. Al final es como asignar un valor a una nueva variable también.

```rust
#[derive(Debug)]
struct Point(i32);

fn foo(x: Point) {
}

fn main() {
    let p = Point(5);
    foo(p);
    println!("{:?}", p);
    // No compila :(
}
```
---
class: middle

# Referencias

Es posible crear referencias a valores en memoria para evitar moverlos

```rust
#[derive(Debug)]
struct Point(i32);

fn foo(x: &Point) {
}

fn main() {
    let p = Point(5);
    foo(&p);
    println!("{:?}", p);
    // Point(5)
}
```

Una referencia mutable es especificada como `&mut T` donde T es el tipo de dato
---
class: middle
## Reglas de las referencias

En algún momento del ciclo de vida de un valor se cumple una y exactamente una de estas afirmaciones:

* Todas las referencias al valor son inmutables
* Existe solo una referencia mutable

Se verifica en tiempo de compilación y esto previene condiciones de carrera.

---
class: center, middle
## Tiempos de vida

En rust esto es cierto que:

**todas las referencias son válidas mientras existan**

¿Cómo se puede asegurar esto en tiempo de compilación?

---
class: middle

khastapasanda?

```rust
fn cadena_mas_larga(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = String::from("hola");
    let c;

    {
        let b = String::from("mundo");
        c = cadena_mas_larga(&a, &b);
        println!("Comparando '{}' y '{}'", a, b);
    }

    println!("'{}' es la cadena más larga", c);
}
```
---
class: middle

Necesitamos especificar tiempos de vida

```rust
fn cadena_mas_larga<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = String::from("hola");
    let c;

    {
        let b = String::from("mundo");
        c = cadena_mas_larga(&a, &b);
        println!("Comparando '{}' y '{}'", a, b);
    }

    println!("'{}' es la cadena más larga", c);
}
```

para conocer el verdadero error con este código

---
class: middle

```
error[E0597]: `b` does not live long enough
  --> cosa.rs:15:35
   |
15 |         c = cadena_mas_larga(&a, &b);
   |                                   ^ borrowed value does not live long enough
16 |         println!("Comparando '{}' y '{}'", a, b);
17 |     }
   |     - `b` dropped here while still borrowed
...
20 | }
   | - borrowed value needs to live until here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
```

---
class: center, middle

## Consejo

**Todo el asunto de los tiempos de vida es asociar la vida de referencias con la vida de otros objetos que conocemos**

## Ejercicios

http://www.rust-tutorials.com/RustConf17/
