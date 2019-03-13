struct Area {
    largo: f32,
    ancho: f32
}

struct Ventana {
    tipo: String,
    tiene_cortinas: bool,
}

struct Habitacion {
    numero_de_puertas: i32,
    ventanas: Vec<Ventana>,
    nombre_del_cuarto: String,
    hay_guardaropa: bool, // true = si
    area: Area
}

struct Casa {
    habitaciones: Vec<Habitacion>
}

fn main() {
    let habitacion_principal = Habitacion {
        numero_de_puertas: 1,
        ventanas: vec![
            Ventana {
                tipo: String::from("Rectangular"),
                tiene_cortinas: true
            },
            Ventana {
                tipo: String::from("Cuadrada"),
                tiene_cortinas: true
            },
        ],
        nombre_del_cuarto: String::from("Habitación principal"),
        hay_guardaropa: true,
        area: Area { largo: 24.2, ancho: 32.3 }
    };

    let banio = Habitacion {
        numero_de_puertas: 1,
        ventanas: vec![
            Ventana {
                tipo: String::from("Cuadrada"),
                tiene_cortinas: false
            },
        ],
        nombre_del_cuarto: String::from("Baño"),
        hay_guardaropa: false,
        area: Area { largo: 20.0, ancho: 15.2 }
    };

    println!("{}",area_habitacion(&banio.area));

    let casa_del_gatito = Casa {
        habitaciones: vec![
            banio, 
            habitacion_principal
        ]
    };
    println!("{}",area_habitacion(&casa_del_gatito.habitaciones[0].area));
}

fn area_habitacion(area: &Area) -> f32 {
    area.largo * area.ancho
}