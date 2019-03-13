fn main(){
    let color_favorito: Option<&str> = Some("Purple");

    if let Some(color) = color_favorito{
        println!("Usando tu color favorito, {} como color de fondo", color);
    } 
}