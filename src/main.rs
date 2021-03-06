use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_CONST: i32 = 5;

struct User<'a> {
    username: &'a str,
    age: u32,
}

fn main() {
    let mut user = User {
        username: "Jose Jalapeno",
        age: 39,
    };

    println!("Guess the number!");

    let secret_cislo = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_cislo);
    let mut pocet_pokusu = 0;

    println!("Give your name");

    let mut username = String::new();

    match io::stdin().read_line(&mut username) {
        Ok(_) => user.username = username.trim(),
        Err(_) => {
            println!("Bad input you sun");
            panic!();
        }
    };

    println!(
        "Welcome {} jsi stary jako prase, presne: {} let",
        user.username, user.age
    );
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

        let guess = guess
                        .trim()
                        .parse::<f64>()
                        .expect("Parsing ERROR!")
                        .round() as i32;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_cislo) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN");
                break;
            }
        }

        pocet_pokusu += 1;
        println!("Remain number of tries: {}", MAX_CONST - pocet_pokusu);
    }
}
