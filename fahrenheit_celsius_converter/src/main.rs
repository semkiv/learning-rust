use std::io;

fn main() {
    let fahrenheits: f64 = loop
    {
        println!("Enter tempreture (°F):");

        let mut fahrenheits = String::new();
        io::stdin()
            .read_line(&mut fahrenheits)
            .expect("Failed to read the input!");

        let fahrenheits = fahrenheits.trim();

        match fahrenheits.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("{} is not a valid temperature!", fahrenheits),
        };
    };

    println!("{:.1}°F is {:.1}°C", fahrenheits, fahrenheit_to_celsius(fahrenheits));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
