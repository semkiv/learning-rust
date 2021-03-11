// we don’t need to annotate any code in `tests` with `#[cfg(test)]` - Cargo treats the tests directory specially and compiles files in this directory only when we run `cargo test`

use rectangles::Rectangle; // each file in the `tests` directory is a separate crate, so we need to bring our library into each test crate’s scope

mod common;
#[test]
fn two_separate_rectangles_sharing_area() {
    common::setup();

    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(50, 4);

    assert_eq!(rect1.area(), rect2.area());
}
