#[derive(Debug)]
struct Rectangulo {
    largo: u32,
    ancho: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.largo
    }
    fn cuadrado(tamanio: u32) -> Rectangulo {
        Rectangulo { ancho: tamanio, largo: tamanio }
    }
}

fn main() {
    let cuadrado_2 = Rectangulo::cuadrado(3); 
    println!("El área del rectangulo es {}",cuadrado_2.area());
    println!("Rectangulo cuadrado: {:?}",cuadrado_2);
}