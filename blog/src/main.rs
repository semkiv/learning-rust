use blog::{Post, RustifiedPost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(". And for dinner too.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today. And for dinner too.", post.content());

    let mut post = RustifiedPost::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review(); // notice that we need to re-declare `post` after each transformation; that's because the methods that transform it consume `self` which in turn is to make sure that the old cannot be used after it has been transformed into a new one
    let mut post = post.reject();
    post.add_text(". And for dinner too.");
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today. And for dinner too.", post.content());
}
