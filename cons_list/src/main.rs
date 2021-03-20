use cons_list::{List, SharedList};
use std::rc::Rc;

fn main() {
    let mut list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    List::for_each_mut(&mut list, |x| {
        *x *= *x;
    });

    print!("list: ");
    List::for_each(&list, |x| {
        print!("{} ", x);
    });
    println!("");

    let shared_list1 = Rc::new(SharedList::Cons(
            1,
            Rc::new(SharedList::Cons(
                2,
                Rc::new(SharedList::Cons(3, Rc::new(SharedList::Nil))),
            )),
        ));

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
