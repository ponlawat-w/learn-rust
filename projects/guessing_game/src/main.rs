use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u8 = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u8 = match guess.trim()
            .parse::<u8>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Unable to parse your message into a number.");
                    continue;
                }
            };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{guess} is less than the secret number."),
            std::cmp::Ordering::Greater => println!("{guess} is greater than the secret number."),
            std::cmp::Ordering::Equal => {
                println!("{guess} is the secret number!");
                break;
            }
        }
    }
}
