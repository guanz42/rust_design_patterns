#![allow(dead_code)]

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("[Draft] request_review");
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("[Draft] approve");
        self
    }
}

struct PendingReview;
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("[PendingReview] request_review");
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("[PendingReview] approve");
        Box::new(Published {})
    }
}

struct Published;
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("[Published] request_review");
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("[Published] approve");
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        let text = "State is a behavioral design pattern";
        post.add_text(text);
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(text, post.content());
        println!("post content: {}", post.content());
    }
}
