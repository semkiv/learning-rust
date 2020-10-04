use rectangles::Rectangle;

mod common;

#[test]
fn two_separate_rectangles_sharing_area() {
    common::setup();

    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(50, 4);

    assert_eq!(rect1.area(), rect2.area());
}