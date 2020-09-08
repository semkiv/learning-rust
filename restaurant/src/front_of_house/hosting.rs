pub use hosting::add_to_waitlist;
pub use hosting::seat_at_table;

pub mod hosting { // modules can contain other modules; definitions inside a module are private by default, they can be exposed using `pub` keyword
    pub fn add_to_waitlist() {} // making a module public doesn't expose its contents, each of its children has to be exposed separately

    pub fn seat_at_table() {}
}
