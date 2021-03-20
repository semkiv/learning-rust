pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct ConsolePrinter {}

impl ConsolePrinter {
    pub fn new() -> ConsolePrinter {
        ConsolePrinter {}
    }
}

impl Messenger for ConsolePrinter {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // `MockMessenger` is a mock object that is used in testing, it logs all the messages being sent.
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // since `send` method requires immutable `&self` we cannot mutate the vector directly; instead we use `RefCell` that allows for so-called interior mutability
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // `RefCell` has `borrow` and `borrow_mut` methods to access the underlying value; it keeps track of the calls to these methods in runtime and panics when on attempt to create references that would violate borrowing rules
        }
    }

    #[test]
    fn send_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(
            *mock_messenger.sent_messages.borrow(),
            vec![String::from(
                "Warning: You've used up over 75% of your quota!"
            )]
        );
    }

    #[test]
    fn send_an_over_90_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(95);

        assert_eq!(
            *mock_messenger.sent_messages.borrow(),
            vec![String::from(
                "Urgent warning: You've used up over 90% of your quota!"
            )]
        );
    }

    #[test]
    fn send_an_over_100_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(105);

        assert_eq!(
            *mock_messenger.sent_messages.borrow(),
            vec![String::from("Error: You are over your quota!")]
        );
    }
}
