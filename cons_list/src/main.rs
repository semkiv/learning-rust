use cons_list::{list, List, shared_list, SharedList};
use std::rc::Rc;

fn main() {
    let mut list = list!(1, 2, 3);

    List::for_each_mut(&mut list, |x| {
        *x *= *x;
    });

    print!("list: ");
    List::for_each(&list, |x| {
        print!("{} ", x);
    });
    println!("");

    let shared_list1 = shared_list!(1, 2, 3);

    print!("shared_list1: ");
    SharedList::for_each(&shared_list1, |x| {
        print!("{} ", x);
    });
    println!("");

    let shared_list2 = Rc::new(SharedList::Cons(20, Rc::clone(&shared_list1)));
    let shared_list3 = Rc::new(SharedList::Cons(30, Rc::clone(&shared_list1)));

    print!("shared_list2: ");
    SharedList::for_each(&shared_list2, |x| {
        print!("{} ", x);
    });
    println!("");
    print!("shared_list3: ");
    SharedList::for_each(&shared_list3, |x| {
        print!("{} ", x);
    });
    println!("");
}
