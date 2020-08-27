fn main() {
    let user1 = User {
        email: String::from("user@example.com"), // fields does not necsarily have to be initialized in the same order as they were declared
        username: String::from("user123"),
        is_active: true,
        sign_in_count: 0
    };

    print_user(&user1);

    let user2 = build_user(String::from("test@test.com"), user1.username);

    print_user(&user2);

    let user3 = User {
        email: String::from("anotheruser@example.com"),
        username: String::from("anotheruser123"),
        ..user1 // the 'struct update syntax' specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance
    };

    print_user(&user3);

    let red = Color(255, 0, 0);
    print_color(&red);
    let origin = Point(0.0, 0.0, 0.0);
    print_point(&origin);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    is_active: bool
}

struct Color(u32, u32, u32); // these are called 'tuple structs'; they are distinct types but don't have named fields and in all other aspects behave identically to the tuples: can be destructured, idividual values can be accessed using .N etc.
struct Point(f64, f64, f64);

fn build_user(email: String, username: String) -> User {
    User {
        email: email, // 'field init shorthand' allows not to repeat the name twice if the field and the parameter name match
        username: username,
        is_active: true,
        sign_in_count: 0
    }
}

fn print_user(user: &User) {
    println!("User {}. Email: {}, active: {}, sign in count: {}.", user.username, user.email, user.is_active, user.sign_in_count);
}

fn print_point(point: &Point) {
    println!("(x: {}, y: {}, z: {})", point.0, point.1, point.2);
}

fn print_color(color: &Color) {
    println!("(r: {}, g: {}, b: {})", color.0, color.1, color.2);
}
