// Ownership rules
// All values have owner 
// Only one owner to one value
// If owner died ther value is dropped 

fn main() {

    let s1 = String::from("Hello");

    let s2 = s1; 

    // println!("{s1}"); The s1 is undefined now because the owner is changed

    println!("{s2}");

    let x = 10;
    let y = x;

    println!("x = {x}, y = {y}"); // X and Y both have the values because i32 are stored in stack and stack values are easy to copy
    // x also have copy trait

    takes_ownershiop(s2);
    
    // println!("{s2}");

    makes_copy(x);

    println!("x = {x}");


    let mut  new_str = String::from("Hello, world");

    borrowing_and_refer(&mut new_str); // mut reference can be borrowed by only one

}

fn takes_ownershiop(some_string : String) {
    println!("{some_string}");
}

fn makes_copy(x : i32) {
    println!("{x}");
}


fn borrowing_and_refer (s : &mut String) {

    s.push_str(". Nice to meet you");
    
    println!("{s}");

}