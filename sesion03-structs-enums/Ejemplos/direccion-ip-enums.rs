#[derive(Debug)]
enum DireccionIp{
	V4(String),
	V6(String),
}
						
fn main(){
    let home = DireccionIp::V4(String::from("127.0.0.1"));
    println!("Home es: {:?}", home);
}