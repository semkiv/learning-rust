use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Meters(u32);

impl Meters {
    pub fn new(length: u32) -> Meters {
        Meters(length)
    }
}

// we can overload operator `+` by implementing `Add` trait for the type; `Add<Rhs=Self>` is a generic trait with the generic type parameter that specifies the type of the other operand; this generic type parameter defaults to `Self`, i.e. the other parameter by default has the same type
impl Add<Meters> for Millimeters {
    type Output = Millimeters; // `Add` trait also has `Output` associated type corresponding to the type of result

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[derive(Debug, PartialEq)]
pub struct Millimeters(u32);

impl Millimeters {
    pub fn new(length: u32) -> Millimeters {
        Millimeters(length)
    }
}

// the fact that we have `T::Add<U>` doesn't mean we get `U::Add<T>` for free, we need to explicitly define it
impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 * 1000 + other.0)
    }
}
