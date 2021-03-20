use mock_objects::{ConsolePrinter, LimitTracker};

fn main() {
    let p = ConsolePrinter::new();
    let mut tracker = LimitTracker::new(&p, 100);
    tracker.set_value(80);
}
