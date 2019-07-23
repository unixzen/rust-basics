extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Число должно быть между 1 и 100, получено {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

//    println!("Загаданное число: {}", secret_number);
    
    loop {
        println!("Пожалуйста, введите предположение.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не удалось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        let guess_item: Guess = Guess::new(guess as u32);
        let guess: u32 = guess_item.value();

        println!("Вы выиграли! {}", guess);

//        if guess < 1 || guess > 100 {
//            println!("Загаданное число должно быть между 1 и 100");
//            continue;
//        }

//        println!("Ваша попытка: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less		=> println!("Слишком маленькое!"),
            Ordering::Greater 	=> println!("Слишком большое!"),
            Ordering::Equal		=> {
                println!("Вы выиграли!");
                break;
            }
        }
    }
}


