
// Polymorphism -> code that can work with data of multiple types but not in rust.
// Rust instead uses generics to abstract over different possible types and trait bounds to impose 
// constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.

pub trait Draw { // This is the trait object
    fn draw(&self);
}

pub struct Screen {
    components : Vec<Box<dyn Draw>> // dyn -> Dynamic
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width : u32,
    height : u32,
    lablel : String
}

impl Draw for Button { // Implimenting in Button so that screen can accept it
    fn draw(&self) {
        println!(" w = {}, h = {}, label = {}", self.width, self.height, self.lablel);
    }
}

pub struct SelectButton {
    width : u32,
    height : u32,
    lablel : Vec<String>
}

impl Draw for SelectButton { // Implimenting in Button so that screen can accept it
    fn draw(&self) {
        println!(" w = {}, h = {}", self.width, self.height);
    }
}

pub fn trait_object_fn() {
    let screen = Screen {
        components : vec![
            Box::new(Button {
                width : 10, height :20, lablel : String::from("Hi")
            }),
            Box::new( SelectButton {
                width : 100, 
                height : 200,
                lablel : vec![
                    String::from("Hello"),
                    String::from("Hi"),
                    String::from("Namaste"),
                ]
            })
        ]
    };

    screen.run();
}