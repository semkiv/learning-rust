use tree::Node;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // in order to use `Weak` we need first `upgrade` it; `upgrade` method returns `Option<T>`

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // we can create `Weak` from `Rc` by downgrading the latter

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
