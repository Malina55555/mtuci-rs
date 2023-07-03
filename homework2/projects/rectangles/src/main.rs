#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	
	fn width(&self) -> bool {
		self.width > 0
	}
	
	fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {	
	fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //let width1 = 30; - без структуры
    //let height1 = 50;
	//let rect1 = (30, 50); - кортеж
	//let scale = 2;
    let rect1 = Rectangle {
		width: 30,
        //width: dbg!(30 * scale),
        height: 50,
    };
	
	let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
	println!(
        "Площадь прямоугольника равна {} квадратных пикселей.",
        //area(width1, height1) - без структуры
		//area(rect1) - кортеж
		//area(&rect1) - через структуру и ф-ю
		rect1.area()
    );
	
	if rect1.width() {
        println!("Прямоугольник имеет ненулевую ширину, равную {}", rect1.width);
    }
	
	println!("rect1 это {:?}", rect1);
	println!("rect1 это {:#?}", rect1);
    //dbg!(&rect1);
	println!("Поместится ли rect2 в rect1? {}", rect1.can_hold(&rect2));
    println!("Поместится ли rect3 в rect1? {}", rect1.can_hold(&rect3));
	
	let sq = Rectangle::square(3);
	println!("sq --- {sq:?}");
}

//fn area(width: u32, height: u32) -> u32 {
    //width * height - без структуры
//fn area(dimensions: (u32, u32)) -> u32 {
  //  dimensions.0 * dimensions.1 - кортеж
//fn area(rectangle: &Rectangle) -> u32 {
  //  rectangle.width * rectangle.height - теперь в impl
//}