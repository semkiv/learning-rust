// Each file in the `tests` directory is compiled as its own separate crate, that's why we need to put the file with common code into a dedicated subfolder. This is an alternate naming convention: `<module_name>/mod.rs` works just as `<module_name.rs>`.

pub fn setup() {
    println!("Hello, world!"); // let's pretend this function does some meaningful preparation common for many tests
}
