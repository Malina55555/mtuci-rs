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
}