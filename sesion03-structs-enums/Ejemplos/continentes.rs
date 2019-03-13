#[derive(Debug)]
enum Continente{
    Europa,
    Asia,
    Africa,
    America,
    Oceania,
}

fn main(){
   let continente = Continente::Asia;
   match continente {
        Continente::Europa => println!("E"),
        Continente::Asia => println!("As"),
        Continente::Africa => println!("Af"),
        Continente::America => println!("Am"),
        Continente::Oceania => println!("O"),
    }
}
