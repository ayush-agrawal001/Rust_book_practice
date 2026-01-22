

fn main() {
    
    // closures are anonymous functions you can save in a variable or pass as arguments to other functions
    
    fn add (x : u32) -> u32 { x + 1 } // An fn Function

    let add_2 = |x : u32| -> u32 { x + 1 }; // A closure function with explicit type anotation

    let add_3 = |y : u32| { y + 1 };
    let add_4 = |y : u32| y + 1 ; // A closure function

    let arr = vec![add(0), add_2(1), add_3(2), add_4(3)];

    println!("{:?}", arr);

}

// An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished
// iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // Iterators and closures 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}