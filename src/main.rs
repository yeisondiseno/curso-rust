fn main() {
    println!("Introduce tu edad: ");
    let mut age: String = String::new();

    // Tomar edad
    std::io::stdin().read_line(&mut age).unwrap();

    // Transformar Edad
    let age_int: u8 = age.trim().parse().unwrap();

    if age_int >= 18 {
        println!("Puedes entrar a la discoteca");
    } else {
        println!("No puedes entrar a la discoteca")
    }

    let mut i: u32 = 30;

    i += 1;

    println!("I es: {}", i)
}
