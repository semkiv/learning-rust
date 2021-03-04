pub use guess::Guess; // crates define their own modules, so without this `use guess::Guess` in main.rs would have to be `use guess::guess::Guess`; this essentially re-exports `Guess` onto the crate's level

pub mod guess {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::guess::Guess;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")] // `should_panic` attribute makes a test pass if the code inside the function panics; the panic message has to contain the `expected` parameter value
    fn greater_than_one_hundred() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn less_than_one() {
        Guess::new(0);
    }
}
