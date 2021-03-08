// the standard library (`std`) is also a crate that’s external to the package; although we don’t need to change `Cargo.toml` to include `std` (because the standard library is shipped with the Rust language), we do need to refer to it with `use` to bring items from there into our package’s scope
use std::collections::HashMap; // when bringing in structs, enums and items other than functions, it's idiomatic to specify the full path to the item

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
