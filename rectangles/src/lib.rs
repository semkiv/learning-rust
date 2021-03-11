#[derive(Debug)] // derived trait required for formatting, automatically generates required methods based on the struct members using the predefined macro
pub struct Rectangle {
    width: u32,
    height: u32,
}

// methods in Rust are defined inside `impl` block
impl Rectangle {
    // methods always take `self` as a first parameter; we still have to use `&` before `self` because methods are generally allowed to take the ownership of `self` (this is rare though, usually only when the method transforms `self` into something else and we want to prevent the caller from using the original instance after the transformation)
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // there might be multiple `impl` blocks
    pub fn print(&self) {
        println!("{:#?}. Area: {} square pixels.", self, self.area());
    }

    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // this is an "associated function"
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(larger.can_hold(&smaller)); // the `assert!` macro is used to ensure that some condition in a test evaluates to true
    }

    #[test]
    // tests can also return `Result<T, E>` and not rely on macros like `assert!`
    fn smaller_cannot_hold_larger() -> Result<(), String> {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        if !smaller.can_hold(&larger) {
            Ok(())
        } else {
            Err(String::from(
                "It should be impossible for a smaller rectangle to hold a larger one.",
            ))
        }
    }

    #[test]
    fn area_is_correct() {
        let rect = Rectangle::new(10, 5);
        assert_eq!(rect.area(), 50); // the `assert_eq!` macro check first its two parameters for equality; `assert_ne!` tests for inequality
    }
}
