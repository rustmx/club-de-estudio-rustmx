fn main() {
    let numero_de_puertas_habitacion: i32 = 1;
    let numero_de_ventas_habitacion: i32 = 2;
    let tipo_de_ventana_habitacion: String = String::from("Rectangulares");
    let tiene_cortinas_habitacion: bool = false; // true = Si
    let nombre_del_cuarto_habitacion: String = String::from("Habitación principal");
    let hay_guardaropa_habitacion: bool = false; // true = si
    let ancho_del_cuarto_habitacion: f32 = 32.2;
    let largo_del_cuarto_habitacion: f32 = 20.0;

    area_cuarto(&ancho_del_cuarto_habitacion, &largo_del_cuarto_habitacion);

    let numero_de_puertas_banio: i32 = 1;
    let numero_de_ventas_banio: i32 = 1;
    let tipo_de_ventana_banio: String =  String::from("Cuadradas");
    let tiene_cortinas_banio: bool = false; // true = Si
    let nombre_del_cuarto_banio: String = String::from("Baño");
    let hay_guardaropa_banio: bool = false; // true = si
    let ancho_del_cuarto_banio: f32 = 12.0;
    let largo_del_cuarto_banio: f32 = 15.0;

    area_cuarto(&ancho_del_cuarto_banio, &largo_del_cuarto_banio);
}

fn area_cuarto(ancho: &f32, largo: &f32) -> f32 {
    ancho * largo
}