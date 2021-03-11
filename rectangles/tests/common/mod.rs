// each crate (source file) in the `tests` directory is treated as a separate integration test
// that's why if we need a crate with just common code we need to put it into a dedicated subfolder
// this is an alternate naming convention: `<module_name>/mod.rs` works just as `<module_name.rs>`
// but naming the file this way tells Rust not to treat the common module as an integration test file

pub fn setup() {
    println!("Hello, world!"); // let's pretend this function does some meaningful preparation common for many tests
}
