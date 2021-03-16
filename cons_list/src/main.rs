use cons_list::List::{Cons, Nil};

fn main() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    cons_list::for_each_mut(&mut list, |x| {
        *x *= *x;
    });
    cons_list::for_each(&list, |x| {
        println!("{}", x);
    });
}
