

// This is a way to ignore the infinite cycle with Weak ref

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
pub struct Node {
    value : i32,
    parent : RefCell<Weak<Node>>,
    child : RefCell<Vec<Rc<Node>>>
}

pub fn tree() {
    
    let leaf = Rc::new(Node {
        value : 3,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![])
    });

    let branch = Rc::new(Node {
        value : 5,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

}