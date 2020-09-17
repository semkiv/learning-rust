use traits::NewsArticle;
use traits::Summary;
use traits::Tweet;

use core::fmt::Debug;
use core::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet. {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    traits::notify(&tweet);
    traits::notify(&article);
}

fn _some_function<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone, // `where` clauses can be used to simplify the signature of complex generic functions; in this particular case the standard signature would look like `fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32`
          U: Clone + Debug,
{
    42
}
