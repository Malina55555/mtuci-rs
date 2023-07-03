//enum IpAddr {
  //  V4(String),
    //V6(String),
//}
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

//struct Ipv4Addr {
    // --snip--
//}
//struct Ipv6Addr {
    // --snip--
//}
//enum IpAddr {
  //  V4(Ipv4Addr),
  //  V6(Ipv6Addr),
//}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
	
//enum Option<T> {
  //  None,
    //Some(T),
//}


//struct IpAddr {
  //      kind: IpAddrKind,
    //    address: String,
//}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}




fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
			println!("LUCKY penny!");
			1
		}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
			println!("State quarter from {:?}!", state);
			25
		}
    }
}

fn main() {
    //let home = IpAddr {
      //  kind: IpAddrKind::V4,
        //address: String::from("127.0.0.1"),
    //};
	//let home = IpAddr::V4(String::from("127.0.0.1"));
	let home = IpAddr::V4(127, 0, 0, 1);
    //let loopback = IpAddr {
      //  kind: IpAddrKind::V6,
        //address: String::from("::1"),
    //};
	//let loopback = IpAddr::V6(String::from("::1"));
	let loopback = IpAddr::V6(String::from("::1"));
	
    //let four = IpAddrKind::V4;
    //let six = IpAddrKind::V6;
	
	let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
	
	let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
	
	let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
		//_ => reroll(),
		_ => (),
    }
	
	let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
	
	let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
	
	let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// fn route(ip_kind: IpAddrKind) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
	
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}