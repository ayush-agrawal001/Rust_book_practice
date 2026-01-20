// To choose a "String" and "&str" in struct. This is a deliberate choice because we want each instance of this struct to own all of its 
// data and for that data to be valid for as long as the entire struct is valid.

struct Rectangle {
    width : i32,
    height : i32,
    name : String,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width : 10,
        height : 20,
        name : String::from("Ayush"),
    };

    let area1 = rect1.area();

    println!("name = {:?} , h = {:?} and w = {:?} and area = {area1}", rect1.name, rect1.height, rect1.width);

}
