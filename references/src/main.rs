fn main() {
    let s1 = String::from("Hello, world!");
    println!("{}", calculate_length(&s1)); // passing the argument by reference

    let mut s2 = String::from("Hello");
    change(&mut s2); // passing the argument by mutable reference
    let r1 = &s2; // there can be multiple immutable references, but...
    let r2 = &s2;
    println!("{} {}", r1, r2);
    let mr1 = &mut s2; // ...we cannot have mutable reference while there's immutable ones (but beacuse references live up to the point of th last usage, which is `println!` call in this particular case, this code compiles)...
    // let mr2 = &mut s2; // ...and there can be only one mutable reference in a given scope
    change(mr1);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize { // this function takes its parameter by reference
    s.len()
}

fn change(s: &mut String) { // this one takes a mutable reference as a parameter
    s.push_str(", world!");
}
