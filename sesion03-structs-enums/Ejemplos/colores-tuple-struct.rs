struct Color(i32, i32, i32);

fn main(){
    let black = Color(0, 0, 0);
	println!("El código del color negro en rgb es: #{}{}{} ", black.0, black.1, black.2);
}