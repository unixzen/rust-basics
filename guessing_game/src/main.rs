extern crate rand;
use std::io;
use rand::Rng;


fn main() {
    println!("Угадайте число!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Загаданное число: {}", secret_number);

    println!("Пожалуйста, введите предположение.");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Не удалось прочитать строку");

    println!("Ваша попытка: {}", guess);
}

