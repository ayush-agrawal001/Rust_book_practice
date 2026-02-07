

// How match looks
// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

// The let statment formally
// let PATTERN = EXPRESSION
// `bind everything to the variable x, whatever the value is.`
//     let (x, y, z) = (1, 2, 3);

// irrefutable ;- Patterns that will match for any possible value passed
// refutable ;- Patterns that can fail to match for some possible value

mod enums_pattern;

use enums_pattern::enum_matching;

fn main() {

    let x = Some(5);
    let y = 10;

    match x {
        Some(10) => println!("1st case printed, {x:?}"),
        Some(y) => println!("2nd case printed, x = {}", y), // Here the imp thing is y is not what appears in the 
        // global y it is the y that is valid under inner scope only
        _ => println!("Default"),

    }

    println!("Global x = {x:?} and glbl y = {} ", y);

    match y {
        1 | 2 | 3 => println!("matched between 1..4"),
        5..=10 => println!("matched 2nd"),
        _ => println!("")
    }

    // Destructuring Structs and using it in match
    let point = Point {
        x : 1,
        y : 0
    };

    let Point {x, y} = point;

    println!("x = {x}, y = {y}");

    match point {
        Point { x : 0, y } => println!("Point lies on y"),
        Point { x, y: 0} => println!("Point lies on x"),
        Point { x, y} => println!("Point dosent lie on axis"),
    }

    enum_matching();
}

struct Point {
    x : i32,
    y : i32
}