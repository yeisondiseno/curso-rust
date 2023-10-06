fn main() {
    println!("Introduce tu nombre:");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    // obtener la edad y convertir esa edad a un número
    println!("Introduce tu edad:");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    // transformamos la edad en número y quitamos saltos y espacio con trim()
    let age_int: u8 = age.trim().parse().unwrap();

    println!("Hola {} de {} años", name, age_int);
}
