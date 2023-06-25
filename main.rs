use std::io; // "импортирование в прелюдию библиотеки" ввода-вывода
use rand::Rng; //а вот это уже библиотека рандома
use std::cmp::Ordering; // для сравнения


fn main() {
    println!("Угадай число!");

	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	// println!("Секретное число: {secret_number}");
	
	loop { // бесконечный цикл

		println!("Пожалуйста, введите своё предположение.");

		let mut guess = String::new(); // let- введение константы let mut- изменяемая переменная guess-имя переменной
		//остальное возвращает  новый экземпляр string, :: -  new ассоциативная ф-я ДЛЯ ТИПА строки. В общем создание пустой строки
		io::stdin() // получаем ф-ю из модуля. Иначе могли бы std::io::stdin. Дескриптор ввода терминала
			.read_line(&mut guess) //read_line — принять все из ввода, сложить это в строку в качестве аргумента. &-ссылка
			.expect("Ошибка чтения"); //обработка сбоя с помощью типа Resalt
			// все 3 строки: io::stdin().read_line(&mut guess).expect("Failed to read line");
			//  Result — это перечисление, часто называемое enum, может находиться в одном из нескольких вариантов (ok, err)
			// У экземпляра Result есть метод expect
			
		// /let guess: u32 = guess.trim().parse().expect("Введите ЧИСЛО, цифрами!"); //Метод trim на экземпляре String удалит пробелы с краёв и энтер.
		// /Метод parse строк преобразует строку в другой тип. 
		
		let guess: u32 = match guess.trim().parse() { //вместо аварийного выхода - игнорирование ошибки
            Ok(num) => num,
            Err(_) => continue, // _ является всеохватывающим выражением.
        };
		
		println!("Вы предположили: {guess}");
		
		match guess.cmp(&secret_number) { // Метод cmp сравнивает два значения.  match, определяющее, какой вариант Ordering был возвращён из вызова cmp.
			Ordering::Less => println!("Слишком мало!"),
			Ordering::Greater => println!("Слишком много!"),
			Ordering::Equal => {
                println!("Угадали!");
                break; // прерывание цикла
            }
		}
	}
}
