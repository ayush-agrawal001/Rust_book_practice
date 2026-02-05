use rand;

fn main() {
    
    let num : u32 = rand::random();
    
    let x = add_one::add_one(num);
    let y = add_two::add_two(num);

    println!("{num} + 1 = {}", x);
    println!("{num} + 2 = {}", y);

}

