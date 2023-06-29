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
}