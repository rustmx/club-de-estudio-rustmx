fn main() {
    let rectangulo = (30,50);
    println!("El área del rectangulo es {}",area(&rectangulo))
}

fn area(rectangulo: &(u32, u32)) -> u32 {
    rectangulo.0 * rectangulo.1
}