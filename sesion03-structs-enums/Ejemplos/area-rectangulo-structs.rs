#[derive(Debug)]
struct Rectangulo {
    largo: u32,
    ancho: u32,
}

fn main() {
    let rectangulo = Rectangulo {
        largo: 30,
        ancho: 50
    };
    println!("El área del rectangulo es {}",area(&rectangulo));
    println!("Rectangulo: {:?}",rectangulo);
}

fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.largo * rectangulo.ancho
}