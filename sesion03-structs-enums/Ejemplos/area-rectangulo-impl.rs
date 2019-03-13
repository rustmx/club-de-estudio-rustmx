#[derive(Debug)]
struct Rectangulo {
    largo: u32,
    ancho: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.largo
    }
}

fn main() {
    let rectangulo = Rectangulo {
        largo: 30,
        ancho: 50
    };
    println!("El área del rectangulo es {}",rectangulo.area());
    println!("Rectangulo: {:?}",rectangulo);
}