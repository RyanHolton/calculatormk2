use std::io;
use num_integer::gcd;

fn main(){
    loop {
        calculation();
    }
}

fn calculation() {
    let mut first_number: String = String::new();
    
    println!("what is your first number?");

    io::stdin()
        .read_line(&mut first_number)
        .expect("failed to read line");

    let first_number: i32 = first_number.trim().parse().expect("please type first number");

    println!("what function do you want to perform");

    let mut opperation: String = String::new();

     io::stdin()
        .read_line(&mut opperation)
        .expect("failed to read line");

    println!("what is your second number?");

    let mut second_number:String = String::new();
    
    io::stdin()
        .read_line(&mut second_number)
        .expect("failed to read line");

    let second_number: i32 = second_number.trim().parse().expect("please type second number");
    if opperation.trim() == "+"{
        let output: i32 = first_number + second_number;
        println!("{first_number} + {second_number} = {output}");
    }
        else if opperation.trim() == "-"{
        let output: i32 = first_number - second_number;
        println!("{first_number} - {second_number} = {output}");
    }
        else if opperation.trim() == "*"{
        let output: i32 = first_number * second_number;
        println!("{first_number} * {second_number} = {output}");
    }
        else if opperation.trim() == "/"{
        let greatestcommondivisor: i32 = gcd(first_number, second_number);
        if {second_number/greatestcommondivisor} > 1 {
        let output = {first_number/greatestcommondivisor}.to_string() + "/" + &{second_number/greatestcommondivisor}.to_string();
        println!("{first_number} / {second_number} = {output}");
        }
        else if {second_number/greatestcommondivisor} == 1 {
        let output: i32 = {first_number/greatestcommondivisor};
        println!("{first_number} / {second_number} = {output}");
        }
        else if second_number == 0 {
        println!("divide by zero error");
        }
        }
        else {
        println!("invalid characters please use 0-9 for numbers and +,-,*,/ for opperators");
        }
}