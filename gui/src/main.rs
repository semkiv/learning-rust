use gui::{Button, Dimensions, Screen, SelectBox};

fn main() {
    let mut screen = Screen::new();

    screen.push(Box::new(Button::new(
        Dimensions {
            width: 50,
            height: 10,
        },
        String::from("OK"),
    )));

    screen.push(Box::new(SelectBox::new(
        Dimensions {
            width: 100,
            height: 75,
        },
        vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Maybe"),
        ],
    )));

    screen.run();
}
