use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut count = 0;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{

        println!("Please input your guess .");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => {
                println!("Too small!");
                count += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                count += 1;
            }, 
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} chances!", count);
                break;
            }
        }
    }
}
