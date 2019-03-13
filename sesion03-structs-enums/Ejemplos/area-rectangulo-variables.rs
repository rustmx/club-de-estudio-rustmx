fn main() {
    let ancho = 30;
    let largo = 50;

    println!("El área del rectangulo es {}",area(&ancho, &largo))
}

fn area(ancho: &u32, largo: &u32) -> u32 {
    ancho * largo
}