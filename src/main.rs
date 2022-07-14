use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    loop {
        println!("Do you wanna play a game? enter y for yes");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        let input = input.trim_end();

        if String::from("Y").eq_ignore_ascii_case(&input) {
            println!("I have a number in mind, it is from 1 and 20, what do you guess it is?");
            play();
        } else {
            println!("Your input was : {}", input);
            break;
        }
    }
}

fn play() {
    let sn = rand::thread_rng().gen_range(1..=20);
    for a in 0..5 {
        let guess: i32 = get_input();

        match guess.cmp(&sn) {
            Ordering::Less => {
                println!("Too small!");
                if a == 4 {
                    println!("You have exhausted all your tries and have not guessed right. You lose!");
                }
                continue;
            }
            Ordering::Greater => {
                println!("Too big!");
                if a == 4 {
                    println!("You have exhausted all your tries and have not guessed right. You lose!");
                }
                continue;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[allow(unreachable_code)]
fn get_input() -> i32 {
    loop {
        let mut input_string: String = String::new();
        io::stdin().read_line(&mut input_string).expect("Could not read line");
        match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("You did not enter a valid number : {}", input_string);
                continue;
            }
        };
    }
}
