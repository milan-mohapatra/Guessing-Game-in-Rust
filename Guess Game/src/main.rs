use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Please Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..101)/*its return a random number between 1 to 100 (can replace 1..=100)*/;

    //println!("The Secret number is  {}", secret_number);

    loop{
        println!("Enter the number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}"/*called place holder*/, guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("WOW!!!!!!!!!!  You Win!");
                break;
            }
        }

        println!("------------------------");
    }
}