const DAYS_COUNT: usize = 12;

fn main() {
    for day in 1..13 {
        println!("{}", generate_verse(day));
    }
}

fn generate_verse(n: u32) -> String {
    const ORDINALS: [&str; DAYS_COUNT] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    format!(
        "On the {} day of Christmas my true love sent to me\n{}{}",
        ORDINALS[(n - 1) as usize],
        generate_gift_list(n),
        if n != DAYS_COUNT as u32 { "\n" } else { "" },
    )
}

fn generate_gift_list(day: u32) -> String {
    const GIFTS: [&str; DAYS_COUNT] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let this_days_gift = GIFTS[(day - 1) as usize];

    if day == 1 {
        capitalize_first_letter(&String::from(this_days_gift))
    } else if day == 2 {
        format!(
            "{},\nAnd {}",
            capitalize_first_letter(&this_days_gift),
            GIFTS[(day - 2) as usize]
        )
    } else {
        format!(
            "{},\n{}",
            capitalize_first_letter(&this_days_gift),
            generate_gift_list(day - 1)
        )
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars_iter = s.chars();
    let first_letter = chars_iter.nth(0).unwrap().to_uppercase();
    format!("{}{}", first_letter, chars_iter.collect::<String>())
}
