#[derive(Debug)] // derived trait required for formatting
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // methods in Rust are defined inside `impl` block
    fn area(&self) -> u32 { // methods always take `self` as a first parameter; we still have to use `&` before `self` because methods are generally allowed to take the ownership of `self` (this is rare though, usually only when the method transforms self into something else and we want to prevent the caller from using the original instance after the transformation)
        self.width * self.height
    }
}

impl Rectangle { // there might be multiple `impl` blocks
    fn print(&self) {
        println!("{:#?}. Area: {} square pixels.", self, self.area());
    }

    fn square(size: u32) -> Rectangle { // this is an 'associated function'
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 60,
    };
    rect.print();

    let square = Rectangle::square(10);
    square.print();
}
