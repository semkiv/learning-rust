pub use hosting::add_to_waitlist;
pub use hosting::seat_at_table; // even though `hosting` is `pub` and its `pub` functions can be accessed directly we need these re-exports for ergonomics: thing is that source files are crates implicitly creating their own modules and `seat_at_table` is defined in yet another module `hosting`, so the user of `seat_at_table` without this re-exporting would have to use `hosting::hosting::seat_at_table()`; the re-export essentially removes one `hosting`

// `hosting` has no reasons to be `pub` other than to illustrate the re-exporting idea above; making a module `pub` only means that those that can refer to its ancestor can refer to it as well, doing so doesn't expose module contents
pub mod hosting {
    pub fn add_to_waitlist() {} // each of module's children has to be exposed separately

    pub fn seat_at_table() {}
}
