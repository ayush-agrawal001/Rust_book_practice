
// Rust working with state-managment

pub struct Post {
    state : Option<Box<dyn State>>,
    content : String,
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(Draft {} )), content: String::from("") }   
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review ( &mut self ) {
        if let Some(s) = self.state.take() { 
            // Take is used from Option as we removed the previous sate and left `None`
            self.state =  Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

}

pub trait State {
    fn request_review(self : Box<Self>) -> Box<dyn State>;
    fn approve(self : Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post : &'a Post) -> &'a str {
        ""
    }
}

pub struct Draft {}

impl State for Draft {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        Box::new(PendingRequest {})
    }

    fn approve (self : Box<Self>) -> Box<dyn State> {
        self
    }
}

pub struct PendingRequest {}

impl State for PendingRequest  {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self : Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self : Box<Self>) -> Box<dyn State> {
        self   
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


pub fn states_impl_main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("'{}'", post.content());

    post.request_review();
    println!("'{}'", post.content());

    post.approve();
    println!("{}", post.content());
}