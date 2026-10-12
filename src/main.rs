use std::io;
use std::io::Write;

fn main() {

    loop {
    let mut inputForNumberOne = String::new();
    let mut operation = String::new();
    let mut inputForNumberTwo = String::new();
    let mut cicle2 = String::new();

    println!("Введите первое число: ");

    io::stdin()
    .read_line(&mut inputForNumberOne)
    .expect("Error #1 (number1)");

    println!("Введите второе число: ");

     io::stdin()
    .read_line(&mut inputForNumberTwo)
    .expect("Error #2 (number2)");

    println!("Введите операцию: ");

    io::stdin()
    .read_line(&mut operation)
    .expect("Error #3 (operation)");

    let numberOne: f64 = inputForNumberOne.trim().parse().expect("Error #4 (parserOne)");
    let numberTwo: f64 = inputForNumberTwo.trim().parse().expect("Error #5 (parserTwo)");
    let operationType: char = operation.chars().next().expect("Error #6 (operation)");

    match operationType {
        '+' => add(numberOne, numberTwo),
        '-' => subtract(numberOne, numberTwo),
        '*' => multiply(numberOne, numberTwo),
        '/' => divide(numberOne, numberTwo),
        '%' => remainder(numberOne, numberTwo),
        _ => println!("Неккоректная операция, из доступных только: '+' и '-'"),
    }

    println!("Продолжаем? (y/n): ");
    io::stdout().flush().unwrap();

    io::stdin()
    .read_line(&mut cicle2)
    .expect("Error...");

    let choice = cicle2.trim().to_lowercase();

    if choice == "n" || choice == "no" {
        println!("Завершение программы...");
        break;
    } else if choice == "y" || choice == "yes" {
        println!("Продолжение...\n");
    } else {
        println!("Неккоректный ответ,попробуйте ввести (y/n)\n");
    }
}

    
}

fn add(numberOne:f64, numberTwo: f64) {
    println!("{}", numberOne + numberTwo)
}

fn subtract(numberOne: f64, numberTwo: f64) {
    println!("{}", numberOne - numberTwo)
}

fn multiply(numberOne: f64, numberTwo: f64) {
    println!("{}", numberOne * numberTwo);
}

fn divide(numberOne: f64, numberTwo: f64) {
    if numberTwo == 0.0 {
        println!("Ошибка, остаток от деления на 0")
    } else {
        println!("{}", numberOne / numberTwo)   
    }
}

fn remainder(numberOne: f64, numberTwo: f64) {
    if numberTwo == 0.0 {
        println!("Ошибка, остаток от деления на 0")
    } else {
        println!("{}", numberOne % numberTwo)   
    }
}