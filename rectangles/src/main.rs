use rectangles::Rectangle;

// Normally Rust projects that provide a binary have a straightforward `src/main.rs` file that calls logic that lives in the `src/lib.rs` file. The crates that are strictly binary (i.e. don't have `src/lib.rs`) can't have integration test created for them: they are meant to be run on their own and not integrate with anything.

fn main() {
    let rect = Rectangle::new(30, 60);
    rect.print();

    let square = Rectangle::square(10);
    square.print();
}
