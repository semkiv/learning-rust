use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>, // we want each node to know its parent; we also want to avoid circular dependencies, taking into account that the parent owns the children, but not otherwise, we `Weak` to link the parent to its children; because we want to manipulate the nodes in the tree freely without having to have the connected nodes mutable we use `RefCell`
    pub children: RefCell<Vec<Rc<Node<T>>>>, // child nodes are placed into a `Vec`
}
