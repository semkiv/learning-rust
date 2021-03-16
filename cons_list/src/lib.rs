//! A very naive cons list implementation.

/// Cons list is a recursive type holding a value and the rest of the list; the last item contains only `Nil` value
pub enum List<T> {
    Cons(T, Box<List<T>>), // `Box<T>` is used as an indirection measure to resolve infinite recursion
    Nil,
}

/// Recursively traverses `list` applying read-only `func` to each item.
///
/// # Examples
/// ```
/// use cons_list::List::{Cons, Nil};
///
/// let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
/// cons_list::for_each(&list, |x| {
///     println!("{}", x);
/// });
/// ```
pub fn for_each<T, F>(list: &List<T>, mut func: F)
where
    F: FnMut(&T),
{
    match list {
        List::Cons(value, list) => {
            func(value);
            for_each(list, func);
        }
        List::Nil => (),
    }
}

// `&T` and `&mut T` are two different types in Rust so whole new function is needed
/// Recursively traverses `list` applying mutating `func` to each item.
///
/// # Examples
/// ```
/// use cons_list::List::{Cons, Nil};
///
/// let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
/// cons_list::for_each_mut(&mut list, |x| {
///     *x = *x * 2;
/// });
/// ```
pub fn for_each_mut<T, F>(list: &mut List<T>, mut func: F)
where
    F: FnMut(&mut T),
{
    match list {
        List::Cons(value, list) => {
            func(value);
            for_each_mut(list, func);
        }
        List::Nil => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn iterating() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let mut s = String::new();

        for_each(&list, |x| {
            s.push_str(&x.to_string());
            s.push(' ');
        });

        assert_eq!(s.trim(), String::from("1 2 3"));
    }

    #[test]
    fn iterating_and_mutating() {
        let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        for_each_mut(&mut list, |x| *x *= *x);

        let mut s = String::new();

        for_each(&list, |x| {
            s.push_str(&x.to_string());
            s.push(' ');
        });

        assert_eq!(s.trim(), String::from("1 4 9"));
    }
}
