fn main() {
    let mut fibo_user = String::new();
    println!("Escribe el número Fibonnaci que necesites:");
    std::io::stdin().read_line(&mut fibo_user).unwrap();

    let fibo_user_int = fibo_user.trim().parse().unwrap();

    println!(
        "Numero fibo de {} es {}",
        fibo_user_int,
        get_fibo_number(fibo_user_int)
    );
}

fn get_fibo_number(n: i64) -> i64 {
    let mut fibo_number: Vec<i64> = Vec::new();
    let mut i = 0;

    while i <= n {
        if i <= 2 {
            fibo_number.push(i);
        } else {
            let i_1 = i - 1;
            let i_2 = i - 2;

            let new_number = fibo_number[i_1 as usize] + fibo_number[i_2 as usize];

            fibo_number.push(new_number);
        }
        i += 1;
    }

    return fibo_number[n as usize];
}
