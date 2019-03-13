#[derive(Debug)]
enum DireccionIp{
    V4(u8, u8, u8, u8),
	V6(String),
}

fn main(){
    let home = DireccionIp::V4(127, 0, 0, 1);
    println!("Home es: {:?}", home);
}