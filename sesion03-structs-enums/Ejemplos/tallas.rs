fn main(){
    let talla_de_camisa = 20;
 
    let tamanio_de_camisa = match talla_de_camisa{
        16 => "S",
        17 | 18 => "M",
        19 ... 21 => "L",
        22 => "XL",
        _ => "No disponible",
    };

    println!("El tamaño de la camisa es: {}", tamanio_de_camisa);
}