pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(&mut self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self.content) // the goal is to keep all the rules defining what content return inside the structs that implement `State`, so we call `content` method on the value in `state` and pass `content` member of `Post`; we call `as_ref` method on the `Option<T>` because we want a reference to the value inside rather than ownership of the value; because `state` is an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn State>>` is returned (if we didn’t call `as_ref`, we would get an error because we can’t move `state` out of the borrowed `&self` of the function parameter
    }

    pub fn request_review(&mut self) {
        // To consume the old state we call `take` method to take the `Some` value out of `state` field and leave a `None` in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out of `Post` (rather than borrowing it) into `State::request_review`.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn add_text(&self, _content: &mut String, _text: &str) {}

    // default implementations of these methods would need to return `self` breaking the trait object safety requirements, thus there's no other choice than duplicating them in the implementation blocks of the particular states
    fn request_review(self: Box<Self>) -> Box<dyn State>; // note that rather than having `self`, `&self`, or `&mut self` as the first parameter of the method, we have `self: Box<Self>`; this syntax means the method is only valid when called on a `Box` holding the type; this syntax takes ownership of `Box<Self>`, invalidating the old state so the state value of the `Post` can transform into a new state
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _content: &'a String) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // requesting a review for a draft that's already pending review has no effect thus returning the current state without changes
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;
        if self.approvals == 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, content: &'a String) -> &'a str {
        &content
    }
}

// `RustifiedPost` is Rust-like approach to implement transition and transformations between states without using total encapsulation.
// Instead states in this approach are encoded as different types.
pub struct RustifiedPost {
    content: String,
}

impl RustifiedPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // methods that transform the post take `self` and consume the instance in order to ensure that the old state cannot be used after the transformation
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> OneApprovalPost {
        OneApprovalPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct OneApprovalPost {
    content: String,
}

impl OneApprovalPost {
    pub fn approve(self) -> RustifiedPost {
        RustifiedPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
