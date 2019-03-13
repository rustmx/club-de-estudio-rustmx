#[derive(Debug)]
enum TipoDireccionIp{
    V4,
	V6,
}

#[derive(Debug)]
struct DireccionIp{
    tipo: TipoDireccionIp,
	direccion: String,
}

fn main(){
    let home = DireccionIp{
	    tipo: TipoDireccionIp::V4,
	    direccion: String::from("127.0.0.1"),
    };

    println!("La direccion IP {} es de la version {:?} ", home.direccion, home.tipo);
}