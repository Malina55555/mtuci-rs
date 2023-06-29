use std::io;

fn main () {
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	
	let mut x = 5;
	println!("Значение х равно: {x}");
	x = 6;
	println!("Значение х равно: {x}");
	
	println!("В трёх часах - {} секунд", THREE_HOURS_IN_SECONDS);
	
	let x = 5;
    let x = x + 1; // затенение
    {
        let x = x * 2;
        println!("Значение x во внутренней области равно: {x}"); //внутреннее затенение
    }
    println!("Значение х равно: {x}"); // внутреннее затенение закончилось
	
	let spaces = "   ";
	println!("Значение spaces равно:{spaces}.");
	// spaces = spaces.len(); - ошибка, нельзя менять тип переменной
    let spaces = spaces.len();
	println!("Значение spaces равно:{spaces}.");
	
	// let guess = "42".parse().expect("Not a number!"); - ошибка, недостаточно инф-ии для определения типа
		
	let x = 2.0; // f64
	println!("Значение х (тип f64) равно: {x}");
    let y: f32 = 3.0; // f32
	println!("Значение у (тип f32) равно: {y}");
	
	// addition
    let sum = 5 + 10;
	println!("5 + 10 =  {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
	println!("95.5 - 4.3 = {difference}");
    // multiplication
    let product = 4 * 30;
	println!("4 * 30 = {product}");
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
	println!("56.7 / 32.2 =  {quotient}");
	println!("-5 / 3 =  {truncated} - целочисленное деление, т.к. аргументы - целые чила, а не с плавающей точкой");
    // remainder
    let remainder = 43 % 5;
	println!("Остаток от 43/5 = {remainder}");
	
	// логический тип
	let t = true;
	println!("Значение t равно: {t}");
    let f: bool = false; // with explicit type annotation
	println!("Значение f равно: {f}");
	
	//символьный тип
	let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
	println!("{},{},{}",c,z,heart_eyed_cat);
	
	//кортеж - можно разные типы данных, но фиксированная длина
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup;
	println!("Значение y из кортежа (500, 6.4, 1): {y}");
	let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
	println!("Кортеж: {five_hundred}, {six_point_four}, {one}");
	
	//массив - один тип данных, фиксированная длина
	let a = [1, 2, 3, 4, 5];
	let mut index = String::new();
	println!("Введите целочисленный индекс (от 0 до 4) для массива.");
	io::stdin()
            .read_line(&mut index)
            .expect("Не удалось прочитать.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Введите ЧИСЛО, цифрами");

    let element = a[index];

    println!("Значение элемента с индексом {index} равно: {element}"); // выведет ошибку при индексе вне [0..4]
}