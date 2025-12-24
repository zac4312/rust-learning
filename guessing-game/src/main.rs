use std::io;
use rand::Rng;

fn round() {

    let mut ans: Vec<String> = Vec::new();

    while ans.len() != 5 {
 
        println!("-------------------------------------"); 
        println!("Guess a number from 1-5!");
        println!("You got 5 tries!!"); 
        
        let num = rand::thread_rng().gen_range(1..=5); 
        let str_num = num.to_string(); println!("Enter a guess:"); 
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess) 
                   .expect("Wrong guess"); 
            
        if guess.trim() != str_num {
                println!("Incorrect!"); 
                println!("The number was: {str_num}");
                 ans.push(guess.trim().to_string());
        }

        else if guess.trim() == str_num {
            ans.push(guess.trim().to_string());
            println!("Correct!");
            println!("The number was: {str_num}");
            println!("{:?}", ans);

            break;
        }
    }
        
    if ans.len() == 5 {         
        
        println!("game over"); 
        println!("{:?}", ans);
    }
}   

fn main() { 
   
round();

}

