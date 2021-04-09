//! A very naive cons list implementation.
//! Cons list is a recursive type holding a value and the rest of the list; the last item contains only `Nil` value

use std::rc::Rc;

/// Owns the "rest" part of the list. Any item in the list can be mutated as long as the list itself is mutable.
pub enum List<T> {
    Cons(T, Box<List<T>>), // `Box<T>` is used as an indirection measure to resolve infinite recursion
    Nil,
}

/// Immutably references the "rest" part. The items cannot therefore be mutated. On the flip side it allows for having multiple lists reference to the single common tail.
/// ```text
/// list1(value1, |)
///               |---->list3(...)
/// list2(value2, |)
/// ```
pub enum SharedList<T> {
    Cons(T, Rc<SharedList<T>>), // `Rc<T>` is a smart pointer with a reference counter that drops the resource when the counter reaches zero; it allows you to share data between multiple parts of the program for reading only, allowing multiple mutable reference might violate Rust's borrowing rules.
    Nil,
}

impl<T> List<T> {
    /// Recursively traverses `list` applying read-only `func` to each item.
    ///
    /// # Examples
    /// ```
    /// use cons_list::List::{self, Cons, Nil};
    ///
    /// let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    /// List::for_each(&list, |x| {
    ///     println!("{}", x);
    /// });
    /// ```
    pub fn for_each<F>(list: &List<T>, mut func: F)
    where
        F: FnMut(&T),
    {
        match list {
            List::Cons(value, list) => {
                func(value);
                List::for_each(&list, func);
            }
            List::Nil => (),
        }
    }

    // `&T` and `&mut T` are two different types in Rust so whole new function is needed
    /// Recursively traverses `list` applying mutating `func` to each item.
    ///
    /// # Examples
    /// ```
    /// use cons_list::List::{self, Cons, Nil};
    ///
    /// let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    /// List::for_each_mut(&mut list, |x| {
    ///     *x = *x * 2;
    /// });
    /// ```
    pub fn for_each_mut<F>(list: &mut List<T>, mut func: F)
    where
        F: FnMut(&mut T),
    {
        match list {
            List::Cons(value, list) => {
                func(value);
                List::for_each_mut(list, func);
            }
            List::Nil => (),
        }
    }
}

impl<T> SharedList<T> {
    /// Recursively traverses `list` applying read-only `func` to each item.
    ///
    /// # Examples
    /// ```
    /// use cons_list::SharedList::{self, Cons, Nil};
    /// use std::rc::Rc;
    ///
    /// let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    /// SharedList::for_each(&list, |x| {
    ///     println!("{}", x);
    /// });
    /// ```
    pub fn for_each<F>(list: &SharedList<T>, mut func: F)
    where
        F: FnMut(&T),
    {
        match list {
            SharedList::Cons(value, list) => {
                func(value);
                SharedList::for_each(list, func);
            }
            SharedList::Nil => (),
        }
    }
}

/// A macro to facilitate creation of `List`s.
///
/// # Examples
/// ```
/// use cons_list::{list, List};
///
/// let constructed_explicitly = List::Cons(1, Box::new(
///     List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))
/// ));
/// let constructed_with_macro = list!(1, 2, 3);
/// ```
#[macro_export]
macro_rules! list {
    () => (
        List::Nil
    );
    ( $head:expr $( , $tail:expr )* ) => (
        List::Cons($head, Box::new(list!( $( $tail ),* )))
    )
}

/// A macro to facilitate creation of `SharedList`s.
///
/// # Examples
/// ```
/// use cons_list::{shared_list, SharedList};
///
/// use std::rc::Rc;
///
/// let constructed_explicitly = Rc::new(SharedList::Cons(
///     1,
///     Rc::new(SharedList::Cons(
///         2,
///         Rc::new(SharedList::Cons(3, Rc::new(SharedList::Nil))),
///     )),
/// ));
/// let constructed_with_macro = shared_list!(1, 2, 3);
/// ```
#[macro_export]
macro_rules! shared_list {
    () => (
        Rc::new(SharedList::Nil)
    );
    ( $head:expr $( , $tail:expr )* ) => (
        Rc::new(SharedList::Cons($head, shared_list!( $( $tail ),* )))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterating_list() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        let mut s = String::new();

        List::for_each(&list, |x| {
            s.push_str(&x.to_string());
            s.push(' ');
        });

        assert_eq!(s.trim(), String::from("1 2 3"));
    }

    #[test]
    fn iterating_and_mutating_list() {
        let mut list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        List::for_each_mut(&mut list, |x| *x *= *x);

        let mut s = String::new();

        List::for_each(&list, |x| {
            s.push_str(&x.to_string());
            s.push(' ');
        });

        assert_eq!(s.trim(), String::from("1 4 9"));
    }

    #[test]
    fn iterating_shared_list() {
        let list1 = Rc::new(SharedList::Cons(
            1,
            Rc::new(SharedList::Cons(
                2,
                Rc::new(SharedList::Cons(3, Rc::new(SharedList::Nil))),
            )),
        ));
        let list2 = Rc::new(SharedList::Cons(20, Rc::clone(&list1)));
        let mut s1 = String::new();

        SharedList::for_each(&list2, |x| {
            s1.push_str(&x.to_string());
            s1.push(' ');
        });

        let list3 = Rc::new(SharedList::Cons(30, Rc::clone(&list1)));
        let mut s2 = String::new();

        SharedList::for_each(&list3, |x| {
            s2.push_str(&x.to_string());
            s2.push(' ');
        });

        assert_eq!(s1.trim(), String::from("20 1 2 3"));
        assert_eq!(s2.trim(), String::from("30 1 2 3"));
    }
}
