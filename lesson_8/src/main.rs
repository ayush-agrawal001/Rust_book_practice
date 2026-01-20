use std::collections::HashMap;



fn main() {

    //////////// Vectors
    
    let _x : Vec<i32> = Vec::new();

    let x = vec![1, 2, 3, 4, 5];

    let fourth = &x[3]; // This will give error if there is no element in the 4th index

    println!("The Third element is {fourth}");

    let third = x.get(2);


    match third {
        Some(t3rd) => println!("The Third element is {t3rd}"),
        None => println!("No elment at 3rd")
    };


    //loops

    let mut z : Vec<i32> = Vec::new();
    for i in x {
        let y = i * 50;

        z.push(y);
    }
    println!("{:?}", z);

    //////////////////////

    //////////// Enums
    
    #[derive(Debug)]
    enum DiffTypes {
        Int(i32),
        Stringss(String),
        Floatss(f64),
    }

    let vec1 = vec![
        DiffTypes::Int(3),
        DiffTypes::Stringss(String::from("Ayush")),
        DiffTypes::Floatss(2.1),
    ];


    println!("{:?}", vec1);
    //////////////////

    //////////// String and String literal
    
    let s1 = String::from("Ayush");
    let s2 = String::from(" Agrawal");

    let s3 = s1 + &s2; // add operation takes the ownership of s1 and s2 is in 
    // string slice form because add operation parameters

    println!("{s3}");

    ///////////////////

    //////////// Hashmaps
    
    let text = "Ayush Agrawal Ayush Ayush the the what the";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        
        let count = map.entry(word).or_insert(0);

        *count += 1; // This is the dereference to the actual value. If this changes
        //The actual value will also change.

    }

    println!("{map:?}");
    /////////////////////

    
    


    println!("Hello, world!");
}
