enum Mensaje {
    Abandonar,
	Mover { x: i32, y: i32},
	Escribir(String),
	CambiarColor(i32, i32, i32),
}

impl Mensaje{
    fn llamar(&self){
	    println!("Llamando..."); 
    }
}

fn main(){
    let m = Mensaje::Escribir(String::from("Hello"));
	m.llamar();
}