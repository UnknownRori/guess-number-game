use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");
    println!("Please input your guess");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess_parse: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Your input is {} ", guess_parse);

        match guess_parse.cmp(&secret_number) {
            Ordering::Less => print!("Too small! \n"),
            Ordering::Greater => print!("Too big! \n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
