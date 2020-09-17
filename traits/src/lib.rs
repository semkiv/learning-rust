// A trait definition.
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // default implementation (optional)
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// An implementation of the trait.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        self.author.to_owned()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// A function that takes a traits as a parameter.
// The `impl` keyword together with the trait name can be used for this.
// However it is just a syntactic sugar for an equivalent longer form:
// `fn notify<T: Summary>(item: &T)`.
// The longer form might be actually more compact if we have multiple repeating traits.
// Multiple traits can be specified with the `+` syntax, e.g. `&(impl Summary + Display)` or, equivalently, `T: Summary + Display`.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Functions can return traits.
// This is very handy in iterators and closures implementations.
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
