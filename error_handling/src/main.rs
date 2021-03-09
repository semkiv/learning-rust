use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write}; // `self` can be used in nested paths to import the module along with some of its children; here it imports `std::io`, `std::io::ErrorKind`, `std::io::Read` and `std::io::Write`

fn main() {
    let filename = "hello.txt";
    let mut f = match OpenOptions::new().read(true).write(true).open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // different sorts of errors can be handled differently
            ErrorKind::NotFound => match File::create(filename) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problem opening the file: {:?}.", e),
            },
            other_error => panic!("Problem opening the file: {:?}.", other_error),
        },
    };

    f.write_all(b"Test")
        .expect("Problem writing the username to the file."); // `expect` method of `Result<T, E>` ensures that we `panic` with the given error message in case of an error (a similar alternative is `unwrap` method that just `panic`s, without any specific message)

    match read_username_from_file() {
        Ok(s) => println!("Username is {}.", s),
        Err(e) => panic!("Problem reading the username: {:?}.", e),
    }

    panic!("crash and burn");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // the following line that contains `?` operators is essentially equivalent to the following code:
    // ```
    // match File::open("hello.txt") {
    //     Ok(mut f) => {
    //         match f.read_to_string(&mut s) {
    //             Ok(_) => Ok(s),
    //             Err(e) => Err(e),
    //         }
    //     },
    //     Err(e) => return Err(e),
    // };
    // ```
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
