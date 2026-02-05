/// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
/// The Deref trait allows an instance of the smart pointer struct to behave like a reference so that you 
/// can write your code to work with either references or smart pointers. The Drop trait allows you to customize 
/// the code that’s run when an instance of the smart pointer goes out of scope. 


// Box<T> this helps in storing any value in the heap (Checked at compiler time)

// Rc<T> this increases the numbers of owners by increasing the reference pointers (Only immutable ref are allowed)

// Refcell<T> this creates interior mutablity for a reference, interior mutablity means it is mutable only under its method (Checked at runtime) 

mod refcell_pract;
mod reference_cycle;
mod tree;

use reference_cycle::create_cycle;
use tree::tree;

// A recursive type (As dont know the space of List here)
#[allow(dead_code)]
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Cons -> A type of List
    Nil,
}

#[allow(dead_code)]
#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>), // Cons -> A type of List
    Nil2,
}

use std::rc::{Rc};

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};
fn main() {

    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = Rc::new(Cons2(8, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons2(6, Rc::clone(&a)));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let c = Rc::new(Cons2(4, Rc::clone(&b)));
    println!("count after creating a = {} and b = {}", Rc::strong_count(&a), Rc::strong_count(&b));

    dbg!(list);

    dbg!(c);

    mutable_refcell();

    create_cycle();

    tree();
}

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3
}

use crate::List3::{Cons3, Nil3};
use std::cell::RefCell;

fn mutable_refcell() {
    
    let refcell_rc_cons = Rc::new(Cons3(Rc::new(RefCell::new(20)), Rc::new(Nil3)));

    let value  = Rc::new(RefCell::new(100)); // New value defined in list3

    let cons_extended = Cons3(Rc::clone(&value), refcell_rc_cons);

    // dbg!(cons_extended);

    *value.borrow_mut() += 10;

    dbg!(cons_extended);

}