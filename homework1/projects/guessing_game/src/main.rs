use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Игра угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
	
	//println!("Секретное число: {secret_number}");
	
    loop {
        println!("Пожалуйста, напишите ваше предположение.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы предположили: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком велико!"),
            Ordering::Equal => {
                println!("Победа!");
                break;
            }
        }
    }
}