use std::thread;

fn main() {
    //let expensive_closure = |num: u32| -> u32 {
      //  println!("calculating slowly...");
        //thread::sleep(Duration::from_secs(2));
        //num
    //};
	let add_one = |x: u32| x + 1; //<=> fn  add_one_v1   (x: u32) -> u32 { x + 1 }
	
	let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
	
	let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
	
	let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

//impl<T> Option<T> {
//    pub fn unwrap_or_else<F>(self, f: F) -> T
//    where
//        F: FnOnce() -> T
//    {
//        match self {
//            Some(x) => x,
//            None => f(),
//        }
//    }
//}