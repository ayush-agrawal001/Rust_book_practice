use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn create_cycle(){
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial RC count {}", Rc::strong_count(&a));
    println!("a next item, {:?} ", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a RC count after b {}", Rc::strong_count(&a));
    println!("b RC count {}", Rc::strong_count(&b));
    println!("b next item, {:?} ", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a RC count after b {}", Rc::strong_count(&a));
    println!("b RC count after a {}", Rc::strong_count(&b));

    // This create cycle 
    // println!("a next item, {:?}", a.tail())

}