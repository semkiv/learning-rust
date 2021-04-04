pub trait Animal {
    fn baby_name() -> String;
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Human {
    // there's nothing that prevents types implementing traits to have associated functions and/or methods with the same name as the trait they implement has
    pub fn baby_name() -> String {
        String::from("Junior")
    }

    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Animal for Human {
    fn baby_name() -> String {
        String::from("kid")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }

    pub fn fly(&self) {
        println!("🐶");
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Wizard for Dog {
    fn fly(&self) {
        println!("Woof!");
    }
}
