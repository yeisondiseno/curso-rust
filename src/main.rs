use regex::Regex;
use std::io;

// *Note: The packages are obtaining from https://crates.io/

// function
fn multiply(a: i64, b: i64) -> i64 {
    return a * b;
}

fn division(a: i64, b: i64) -> i64 {
    return a / b;
}

fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

fn rest(a: i64, b: i64) -> i64 {
    return a - b;
}

fn create_regex(expression: &str) -> Regex {
    let re = format!(r"(\d+)\s?\{}\s?(\d+)", expression);
    return Regex::new(&re).unwrap();
}

fn create_expression(
    regex: Regex,
    operation_function: impl Fn(i64, i64) -> i64,
    int_expression: String,
) -> String {
    loop {
        // Operations
        let caps = regex.captures(int_expression.as_str());

        if caps.is_none() {
            return int_expression;
        }

        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
        let result = operation_function(left_value, right_value);

        return int_expression.replace(cap_expression, &result.to_string());
    }
}

fn main() {
    // Regex
    let re_add = create_regex("+");
    let re_res = create_regex("-");
    let re_mul = create_regex("*");
    let re_div = create_regex("/");

    // User Data
    println!("Introduce tu expresión");
    let mut expression: String = String::new();
    io::stdin().read_line(&mut expression).unwrap();

    // multiplication
    expression = create_expression(re_mul, multiply, expression);

    // Division
    expression = create_expression(re_div, division, expression);

    // sum
    expression = create_expression(re_add, sum, expression);

    // Rest
    expression = create_expression(re_res, rest, expression);

    // Result
    println!("resultado: {expression}")
}

// In Rust the type null doesn't exist.
// Option<T> = optional type.
// Result<T,E> = T Type E Error.
