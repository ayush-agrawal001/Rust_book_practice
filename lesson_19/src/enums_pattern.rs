
enum Message {
    Quit,
    Move { x : i32, y : i32 },
    Write(String),
    ColorChange(i32, i32, i32)
}

pub fn enum_matching() {
    let msg = Message::ColorChange(121, 12, 2);

    match msg {
        Message::Quit => println!("quit arrived"),
        Message::Write(text) => println!("Some {text}"),
        Message::Move { x, y } => println!("x = {x}, y = {y}"),
        Message::ColorChange(x, y, z) => println!("{x}, {y}, {z}")
    }

    let msg_1 = Message::Move { x: 1, y: 0 };

    match msg_1 {
        Message::Move { x: id @ 1..=7, y } => {
            println!("Found an id in range: {id}")
        }
        Message::Move { x: 10..=12, y } => {
            println!("Found an id in another range")
        }
        Message::Move { x, y} => println!("Found some other id: {x}, {y}"),

        _ => println!("No pattern matched")
    }
}


