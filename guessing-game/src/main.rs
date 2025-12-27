use std::io;
use rand::Rng;

fn round() {

    let mut ans: Vec<String> = Vec::new();

    let mut attempt_count = 0;

    while attempt_count !=  5 {

        attempt_count += 1; 

        println!("-------------------------------------"); 
        println!("You got 5 tries!!"); 
        println!("Attempt Number: {}", attempt_count);
        println!("Guess a number from 1-5!");
               
        let num = rand::thread_rng().gen_range(1..=5); 
        let str_num = num.to_string(); println!("Enter a guess:"); 
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess) 
                   .expect("Wrong guess"); 
        ans.push(guess.clone());

        if guess.trim() == str_num {
            println!("-------------------------------------"); 
            println!("-------------------------------------"); 
            println!("Corrrect!"); 
            println!("You guess: {}", guess.trim()); 
            println!("Answer: {str_num}");
            println!("{:?}", ans);
            break;

        } else { println!("INCORRECT!!!"); 
                 println!("The number was: {str_num}"); 
               }
    }

        if attempt_count == 5 {
            println!("-------------------------------------"); 
            println!("Game Over");
            println!("Guesses:");
            println!("{:?}", ans);
        }
}

fn main() { 
    
    let mut choice = String::from("y");
    let mut round_count = 0;

    while choice.trim() == "y" {
        round_count +=1;
        println!("--------------");
        println!("Round: {}", round_count);
        round();
        println!("Again? (y/n)");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("wrong input");        
    }
}

