use std::io;

fn main() {
    println!("Enter your text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words = input.trim().split_whitespace();

    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    let mut output = Vec::new();

    for w in words {
        let first_char = w.chars().next().unwrap();
        if vowels.contains(&first_char) {
            output.push(format!("{}-hay", w));
        } else {
            let root: String = w.chars().into_iter().skip(1).collect();
            output.push(format!("{}-{}ay", root, first_char));
        }
    }

    println!("{}", output.join(" "));
}
