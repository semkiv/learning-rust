#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house; // load module from a separate file
mod back_of_house;

pub use crate::front_of_house::hosting; // re-exporting the name so the external code could use it

pub fn eat_at_restaurant() {

    crate::front_of_house::hosting::add_to_waitlist(); // Absolute path. Starts from a crate root by using a crate name or a literal `crate`.
    front_of_house::hosting::seat_at_table(); // A relative path. Starts from the current module and uses `self`, `super`, or an identifier in the current module.

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    use front_of_house::serving as srv; // alias; alternatively `self::front_of_house::serving` or `crate::front_of_house::serving`; when bringing in functions it's idiomatic to specify the path up to the parent module
    srv::take_order();
    back_of_house::cook_order();
    srv::serve_order();
    srv::take_payment();
}
