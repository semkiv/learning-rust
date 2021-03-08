pub use serving::*; // a glob operator can be used to re-export all `pub` items from `serving`; these re-exports make `serving`'s (a private module) `pub` functions available from outside

mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}
}
