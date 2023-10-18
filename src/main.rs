#[path = "meta.rs"] mod meta;

use std::io;
use crate::meta::Calc;
use crate::meta::OperationType;

fn main() {
    let calculator: Calc = Calc::new();
    let mut first_number = String::new();
    let mut second_number = String::new();

    println!("Type a first number:");
    io::stdin().read_line(&mut first_number).expect("readline 1 number error !");
    println!("Type a first number:");
    io::stdin().read_line(&mut second_number).expect("readline 2 number error !");

    let first_number :i8 = first_number.trim().parse().expect("parsing 1 num error ! (it's not a number)");
    let second_number :i8 = second_number.trim().parse().expect("parsing 2 num error ! (it's not a number)");
    
    let mut op = String::new();
    println!("Type a math operation:");
    io::stdin().read_line(&mut op).expect("readline operation error !");
    let op:OperationType = match op.trim(){
        "+" => OperationType::Adddition,
        "-" => OperationType::Substraction,
        "*" => OperationType::Multipling,
        "/" => OperationType::Dividing,
        _ => panic!("That's not a math operation"),
    };

    if !calculator.verify_number(first_number){
        panic!("1st number isn't in (1..=10) range");
    }
    if !calculator.verify_number(second_number){
        panic!("2nd number isn't in (1..=10) range");
    }

    // calculating and converting
    let answer = calculator.convert_to_roman_nums(calculator.calculate(first_number, second_number, op));
    
    println!("Roman_nums format output is {}", answer);
}
