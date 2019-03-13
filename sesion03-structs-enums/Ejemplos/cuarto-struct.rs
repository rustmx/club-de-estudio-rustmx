struct Habitacion {
    numero_de_puertas: i32,
    numero_de_ventanas: i32,
    tipo_de_ventana: String,
    tiene_cortinas: bool, // true = Si
    nombre_del_cuarto: String,
    hay_guardaropa: bool, // true = si
    ancho_del_cuarto: f32,
    largo_del_cuarto: f32,
}

fn main() {
    let habitacion_principal = Habitacion {
        numero_de_puertas: 1,
        numero_de_ventanas: 2,
        tipo_de_ventana: String::from("Rectangular"),
        tiene_cortinas: true,
        nombre_del_cuarto: String::from("Habitación principal"),
        hay_guardaropa: true,
        ancho_del_cuarto: 20.0,
        largo_del_cuarto: 32.5
    };

    area_cuarto(&habitacion_principal.ancho_del_cuarto, &habitacion_principal.largo_del_cuarto);

    let banio = Habitacion {
        numero_de_puertas: 1,
        numero_de_ventanas: 1,
        tipo_de_ventana: String::from("Cuadrada"),
        tiene_cortinas: true,
        nombre_del_cuarto: String::from("Baño"),
        hay_guardaropa: false,
        ancho_del_cuarto: 20.0,
        largo_del_cuarto: 15.0
    };

    area_cuarto(&banio.ancho_del_cuarto, &banio.largo_del_cuarto);
}

fn area_cuarto(ancho: &f32, largo: &f32) -> f32 {
    ancho * largo
}