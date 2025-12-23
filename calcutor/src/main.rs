use std::io;

struct Calculator{
    num1: i64,
    num2: i64,
}

impl Calculator {

   fn add(&self) -> i64 { self.num1 + self.num2 }

  fn minus(&self) -> i64 { self.num1 - self.num2 }

  fn multiply(&self) -> i64 { self.num1 * self.num2 }

  fn divide(&self) -> i64 { self.num1 / self.num2 }
} 

fn main() {

    let state = "y";

    while state == "y" {

    let mut calc = Calculator {
        num1: 0,
        num2: 0,
    };


     let mut number1 = String::new();
    println!("Enter Number: ");
    io::stdin().read_line(&mut number1)
               .expect("error");

     let mut number2 = String::new();
    println!("Enter Number: ");
    io::stdin().read_line(&mut number2)
               .expect("error");

    let num1 = number1.trim().parse::<i64>().expect("parse err");
    let num2 = number2.trim().parse::<i64>().expect("parse  err");

    calc.num1 = num1; calc.num2 = num2;

     let mut choice = String::new();
    println!(" | + | - | x | / | ");
    io::stdin().read_line(&mut choice)
               .expect("error");

    

    println!("{}", calc.num1);
    println!("{}", calc.num2);

    match choice.trim() { 
       "+" => println!("{}", calc.add()),
       "-" => println!("{}", calc.minus()), 
       "x" => println!("{}", calc.multiply()), 
       "/" => println!("{}", calc.divide()),
       _ => println!("Invalid iput"),
    }

    }
} 

