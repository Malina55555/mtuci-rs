fn main() {
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
