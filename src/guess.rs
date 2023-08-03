use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

#[allow(dead_code)]
pub fn run() {
    log::info!("guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    log::warn!("The secret number is {secret_number}");
    loop {
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim_end().parse() {
            Ok(num) => num,
            Err(_) => {
                log::warn!("Not a number. Try again");
                continue;
            }
        };

        log::info!("you guessed :-> {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                log::info!("Your guess was correct");
                break;
            }
            _ => log::warn!("Bad guess. Try again"),
        }
    }
}
