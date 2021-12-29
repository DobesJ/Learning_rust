use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX_CONST: i32 = 5;

fn main() {
    println!("Guess the number!");

    let secret_cislo = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_cislo);
    
    let mut pocet_pokusu = 0;

    loop {
    
    if pocet_pokusu == MAX_CONST {
        println!("GAME OVER, cesky jsi v prdeli!");
        break;
    }

    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(text) => text,
        Err(_) => {
            println!("Bad input you sun");
            panic!();
        }
    };
        

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Bad input you Lord of Computer science.");
            continue;
        }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_cislo) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You WIN");
            break;
        }
    }

    pocet_pokusu +=1;
    println!("Remain number of tries: {}", MAX_CONST - pocet_pokusu);
    }

}