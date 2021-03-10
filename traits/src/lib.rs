// a trait definition
// traits that are meant to be implemented by the external code must be marked as `pub`
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // default implementation (optional); can be kept or overridden
    }

    fn summarize_author(&self) -> String; // this function will have to be implemented by all classes that implement `Summary`
}

// a function that takes a traits as a parameter
// the `impl` keyword together with the trait name can be used for this
// however it is just a syntactic sugar for an equivalent longer form:
// `fn notify<T: Summary>(item: &T)`.
// the longer form might be actually more compact if we have multiple repeating traits
// multiple traits can be specified with the `+` syntax, e.g. `&(impl Summary + Display)` or, equivalently, `T: Summary + Display
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// an implementation of the trait
// one restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to the crate
// we can’t implement external traits on external types, for example, we can’t implement the `Display` trait on `Vec<T>`
impl Summary for NewsArticle {
    // `NewsArticle` overrides default implementation of `summarize`
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        self.author.to_owned()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // `Tweet` only implements mandatory `summarize_author` leaving `summarize` implementation default
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// functions can return traits
// this is very handy in iterators and closures implementations
// however, you can only use `impl Trait` if you’re returning a single type, i.e. either `Tweet` or `NewsArticle` in this case, not sometimes one and sometimes another (for example depending on a switch)
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
