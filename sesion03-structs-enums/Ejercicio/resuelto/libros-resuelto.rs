struct Libro{
    nombre: String,
    autor: Autor,
    secciones: Vec<Seccion>,
}

struct Autor{
    nombre: String,
    apellido: String,
}

struct Seccion{
    nombre_de_la_seccion: String,
}

fn main(){
    let libro1 = Libro{
        nombre: String::from("Orgullo y prejuicio"),
        autor: Autor{
            nombre: String::from("Jane"),
            apellido: String::from("Austen"),
         },
        secciones: vec![
            Seccion {
                nombre_de_la_seccion: String::from("Románticas"),
            },
            Seccion {
                nombre_de_la_seccion: String::from("Clásicos"),
            }
        ], 
    };

    println!("El libro {} escrito por {} {} está en la seccion de {}", 
        libro1.nombre, 
        libro1.autor.nombre, 
        libro1.autor.apellido,
        libro1.secciones[0].nombre_de_la_seccion );
}