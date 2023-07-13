//enum Result<T, E> {
  //  Ok(T),
    //Err(E),
//}
use std::fs::File;
use std::fs;
use std::io::{ErrorKind, Read};
use std::io;
use std::error::Error;
use std::net::IpAddr;


fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        //Err(error) => panic!("Problem opening the file: {:?}", error),
		Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
	
	//let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
      //  if error.kind() == ErrorKind::NotFound {
        //    File::create("hello.txt").unwrap_or_else(|error| {
          //      panic!("Problem creating the file: {:?}", error);
            //})
        //} else {
          //  panic!("Problem opening the file: {:?}", error);
        //}
    //});
	
	let greeting_file = File::open("hello.txt").unwrap();
	let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
	
	let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
	
}

fn read_username_from_file() -> Result<String, io::Error> {
	
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


//fn main() -> Result<(), Box<dyn Error>> {
  //  let greeting_file = File::open("hello.txt")?;
  //  Ok(())
//}

// loop {
        // --snip--
        //let guess: i32 = match guess.trim().parse() {
          //  Ok(num) => num,
            //Err(_) => continue,
        //};
        //if guess < 1 || guess > 100 {
          //  println!("The secret number will be between 1 and 100.");
            //continue;
        //}
        //match guess.cmp(&secret_number) {
            // --snip--
    //}
	
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}