use std::io;

struct Calculator{
    num1: i64,
    num2: i64,
}

impl Calculator {

   fn add(&self) -> i64 {
    self.num1 + self.num2
   }

  fn minus(&self) -> i64 {
    self.num1 - self.num2
   }

  fn multipy(&self) -> i64 {
    self.num1 * self.num2
   }

  fn divide(&self) -> i64 {
    self.num1 / self.num2
   }

} 

fn main() {

    let mut calc = Calculator {
        num1: 0,
        num2: 0,
    };

    println!("Enter Number: ");
    io::stdin().read_line(calc.num1)
               .expect("error");

}
