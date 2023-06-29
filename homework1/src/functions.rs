fn main() {
	println!("Привет, мир!");
	another_fn();
	another_function(5);
	 print_labeled_measurement(5, 'h');
	 let y = 6;
	 println!("Значение y равно: {y}");
	 // let x = (let y = 6); - нельзя приравнивать значение к оператору.
	 let y = {
        let x = 3;
        x + 1
    };
    println!("Значение y равно: {y}");
	
	let x = five();
    println!("Значение x равно: {x}");
	
	let x = plus_one(5);
    println!("The value of x is: {x}");
	
}
fn another_fn() {
	println!("Другая ф-я");
}
fn another_function(x: i32) {
    println!("Значение x равно: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Время измерения составляет: {value}{unit_label}");
}
fn five() -> i32 { // <=> let x = 5;
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}