fn main() {
    // dos números
    let num_1 = 120;
    let num_2 = 321;

    let sum = num_1 + num_2;

    loop {
        println!("Escribe la sume de {} y {}", num_1, num_2);

        let mut sum_user = String::new();

        std::io::stdin().read_line(&mut sum_user).unwrap();

        let sum_user_int: i32 = sum_user.trim().parse().unwrap();

        if sum_user_int == sum {
            println!("La suma es correcta.");
            break;
        } else {
            println!("La suma es errada");
        }
    }
}
