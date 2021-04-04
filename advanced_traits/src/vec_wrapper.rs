use std::fmt::{Display, Formatter, Result};
use std::ops::{Deref, DerefMut};

// because of the orphan rule we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate; it’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct
pub struct Wrapper<T>(Vec<T>);

impl<T> Wrapper<T> {
    pub fn new() -> Wrapper<T> {
        Wrapper(Vec::new())
    }
}

// "re-implementing `Display` trait for `Vec`" (technically for `Wrapper`)
impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

// we can implement `Deref` and `DerefMut` traits that dereference `Wrapper` to `Vec<T>` in order to keep all methods of `Vec` for `Wrapper`
impl<T> Deref for Wrapper<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
