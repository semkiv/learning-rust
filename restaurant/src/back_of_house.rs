
pub use back_of_house::Breakfast;
pub use back_of_house::cook_order;

mod back_of_house {
    use crate::front_of_house::serving;

    pub fn cook_order() {}

    fn _fix_incorrect_order() {
        cook_order();
        serving::serve_order();
    }

    pub struct Breakfast {
        pub toast: String, // `struct` fields are private by default and need to be explicitly marked as `pub` when needed
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum _Appetizer {
        Soup, // `enum` variant all become public when the enum itself becomes public
        Salad,
    }
}