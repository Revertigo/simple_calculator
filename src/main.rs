use std::io;
use regex::Regex;

fn main() {
    loop {
        println!("Enter a math expression");
        let mut expression = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut expression).unwrap();
    
        if !validate_expression(&expression) {
            println!("Invalid math expression. Try again");
        } else {
            let result = process_expression(&expression);
            println!("{}", result);
        }
    }
    
}

fn validate_expression(exp: &str) -> bool {
    let re = Regex::new(r"^[1-9]\d*(\.\d+)? *([-+/*] *\d+(\.\d+)?)+").unwrap();
    re.is_match(exp)
}

fn process_expression(exp: &str) -> f64 {
    eval(exp)
}

fn eval(expression: &str) -> f64 {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    let mut current_token = String::new();
    for ch in expression.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            current_token.push(ch);
        } else {
            if !current_token.is_empty() {
                numbers.push(current_token.parse::<f64>().unwrap());
            }
            current_token.clear();
            if !ch.is_whitespace() {
                operators.push(ch.to_string());
            }
        }
    }
    if !current_token.is_empty() {
        numbers.push(current_token.parse::<f64>().unwrap());
    }

    while !operators.is_empty() {
        let operator = operators.pop().unwrap();
        let b = numbers.pop().unwrap();
        let a = numbers.pop().unwrap();
        let result = match operator.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => panic!("Invalid operator: {}", operator),
        };
        numbers.push(result);
    }

    numbers.pop().unwrap()
}