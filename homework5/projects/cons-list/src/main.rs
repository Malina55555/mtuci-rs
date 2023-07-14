enum List {
    Cons(i32, Rc<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::{Cons,Nil};
//use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
	//let list = Cons(1, Cons(2, Cons(3, Nil)));
	//let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
	
	//let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //let b = Cons(3, Rc::clone(&a));
    //let c = Cons(4, Rc::clone(&a));
	
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//fn main() {
  //  let value = Rc::new(RefCell::new(5));
    //let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    //let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    //let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    //*value.borrow_mut() += 10;
    //println!("a after = {:?}", a);
    //println!("b after = {:?}", b);
    //println!("c after = {:?}", c);
//}




//(1,(2(3, Nil))) - cons list   in Lisp

