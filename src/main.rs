use std::io::{stdin,stdout,Write};

fn read(input: &mut String){
   stdout().flush()
   .expect("failed to flush");
   stdin().read_line(input)
   .expect("failed to read");
}

fn main() {
    println!("Hello and Calculate");

    loop{

    println!("Type in you calculation with two numbers");
    let mut num1 = String::new();
    let mut num2 = String::new();
    let  mut operatior = String::new();

    println!("First number?: ");
    read(&mut num1);

    println!("Choose your operator [+-*/]");
    read(&mut operatior);

    println!("Second number?: ");
    read(&mut num2);

    if !num1.trim().chars().all(|chars| chars.is_digit(10)){
        println!("Num 1 has invalid characters, TRY AGAIN");
        continue;
    }        
    if !num2.trim().chars().all(|chars| chars.is_digit(10)){
        println!("Num 2 has invalid characters, TRY AGAIN");
        continue;
    }        
    if operatior.trim().len()>1 {
        println!("an operator can only have 1 character");
        continue;
    }
    let parsed_num1: f32 = num1.trim().parse().unwrap();
    let parsed_num2: f32 = num2.trim().parse().unwrap();
    let parsed_operator : char = operatior.trim().parse().unwrap();

    let operator_choices = String::from("-+*/");

    if !operator_choices.contains(parsed_operator){
        println!("no valid operator");
        continue;
    }

    if operator_choices.contains("/") &&parsed_num2 == 0.0 {
        println!("cant divide zero");
        return;
    }
    let result = match parsed_operator {
        '+' => parsed_num1 + parsed_num2,
        '-' => parsed_num1 - parsed_num2,
        '*' => parsed_num1 * parsed_num2,
        '/' => parsed_num1 / parsed_num2,
        _ => panic!("error in operator")

    };
    println!("The calculation of {} {} {} is {}",parsed_num1,parsed_operator,parsed_num2,result);
    println!("----------------------------------------------------------------------------------");
    println!("");
    }
}
