use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

// we can overload operator `+` by implementing `Add` trait for the type; `Add<Rhs=Self>` is a generic trait with the generic type parameter that specifies the type of the other operand; this generic type parameter defaults to `Self`, i.e. the other parameter by default has the same type
impl Add<Meters> for Millimeters {
    type Output = Millimeters; // `Add` trait also has `Output` associated type corresponding to the type of result

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 * 1000 + other.0)
    }
}

trait Animal {
    fn baby_name() -> String;
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    // there's nothing that prevents types implementing traits to have associated functions and/or methods with the same name as the trait they implement has
    fn baby_name() -> String {
        String::from("Junior")
    }

    fn fly(&self) {
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

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }

    fn fly(&self) {
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

fn main() {
    assert_eq!(Meters(2) + Millimeters(2), Millimeters(2002));

    let person = Human;
    let dogo = Dog;
    person.fly(); // compiler calls `Human::fly` because the receiver (i.e. `&self`) is `&Human` and compiler calls the method implemented directly on the type by default...
    dogo.fly();
    Wizard::fly(&person); // ...but we can explicitly specify that we want `fly` from `Pilot`
    Wizard::fly(&dogo); // compiler infers the implementation to be called (`Human::Wizard::fly` or `Dog::Wizard::fly`) from the type of the receiver

    println!("You can call a kid {}", Human::baby_name());
    println!("You can call a puppy {}", Dog::baby_name());
    // println!(
    //     "A baby human is called a {}",
    //     Animal::baby_name() // compiler cannot infer which implementation to call: `Human::Animal::baby_name` or `Dog::Animal::baby_name`; additional qualification is required
    // );
    println!(
        "A baby human is called a {}",
        <Human as Animal>::baby_name() // this is an example of fully qualified syntax: `<Type as Trait>::function(receiver_if_method, ...);`
    );
}
