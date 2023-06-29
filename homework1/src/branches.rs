fn main() {
	// if
	let number = 3;
    if number < 5 {
        println!("Условие верно");
    } else {
        println!("условие не верно");
    }
	let number = 7;
	if number < 5 {
        println!("Условие верно");
    } else {
        println!("условие не верно");
    }
	
	let number = 3;
    if number != 0 {
        println!("Число не равно 0");
	}	
	
	let number = 6;
    if number % 4 == 0 {
        println!("Число кратно 4");
    } else if number % 3 == 0 {
        println!("Число кратно 3");
    } else if number % 2 == 0 {
        println!("Число кратно 2");
    } else {
        println!("Число не кратно 4, 3, или 2");
    }
	
	let condition = true;
    let number = if condition { 5 } else { 6 };
	// let number = if condition { 5 } else { "six" }; - несовместимые типы значений веток
    println!("The value of number is: {number}");
	
	let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Результат {result}");
}