SIMPLE CLI CALCULATOR TO LEARN RUST STRINGS
===
It uses some new concepts with consoles
```
use std::io::{stdin,stdout,Write};
fn read(input: &mut String){
   stdout().flush()
   .expect("failed to flush");
   stdin().read_line(input)
   .expect("failed to read");
}
    read(&mut num1);
```
uses trims and parse 
```
    if !num1.trim().chars().all(|chars| chars.is_digit(10)){
        println!("Num 1 has invalid characters, TRY AGAIN");
        continue;
    }        
    let parsed_num1: f32 = num1.trim().parse().unwrap();
```
and match
```
    let result = match parsed_operator {
        '+' => parsed_num1 + parsed_num2,
        '-' => parsed_num1 - parsed_num2,
        '*' => parsed_num1 * parsed_num2,
        '/' => parsed_num1 / parsed_num2,
        _ => panic!("error in operator")

```