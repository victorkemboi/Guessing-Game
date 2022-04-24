pub fn get_gamer_tag() -> String {
    let mut gamer_tag = String::new();
    println!("Enter your gamer tag:");

    io::stdin()
        .read_line(&mut gamer_tag)
        .expect("Failed to read gamer tag!");

    gamer_tag
}

pub fn welcome_gamer(gamer_tag: String) {
    println!("Hi {}", gamer_tag);
    println!("Ready up for the Guessing Game!");
    println!()
}

use rand::Rng;
use std::{io, thread, time};

pub fn guessing_game(start: i32, end: i32) {
    println!("Guessing Game");
    println!();
    if start >= end {
        println!("Invalid game setup");
        return;
    }

    let default_pause_time = time::Duration::from_millis(1000);
    let gamer_tag = get_gamer_tag();
    welcome_gamer(gamer_tag);

    thread::sleep(default_pause_time);

    println!("How to Play");
    println!(
        "Enter a guess between {} and {}. 
        A random number is the generated and if your guess is within an acceptable range, you win 🏆.
            Otherwise, you loose 😔",
        start, end
    );
    println!("*** START ***");
    println!();

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let user_guess = guess.trim().parse::<i32>().unwrap();
        let random_number = rand::thread_rng().gen_range(start..end);
        let is_user_correct = if random_number == user_guess {
            "Correct 🥳"
        } else {
            "Incorret 😔"
        };
        println!("Your guess was {}.", is_user_correct);
        thread::sleep(default_pause_time);

        let mut play_again_input = String::new();
        println!("Enter 'y' to play again, 'n' to exit.");

        io::stdin()
            .read_line(&mut play_again_input)
            .expect("Failed to read play again!");

        let play_again = play_again_input.trim() == "y"; // { true } else { false };
        if !play_again {
            break;
        }
    }

    println!("Thanks for playing the Guessing Game. Byeee👋",);
}
