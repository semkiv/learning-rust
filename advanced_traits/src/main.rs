use advanced_traits::{
    beings::{Animal, Dog, Human, Wizard},
    length::{Meters, Millimeters},
    point::{OutlinePrint, Point},
    vec_wrapper::Wrapper,
};

fn main() {
    assert_eq!(Meters::new(2) + Millimeters::new(2), Millimeters::new(2002));

    (Point::new(1, 0) + Point::new(2, 3)).outline_print();

    let mut v = Wrapper::new();
    v.append(&mut vec![1, 2, 3]);
    println!("{}", v);

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
