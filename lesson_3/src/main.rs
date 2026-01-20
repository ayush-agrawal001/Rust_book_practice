fn main() {
    
    // Variables are immutable by default to make it mut use ```let mut var_name = 10```

    let x = 5;
    
    // x = 4 cant do this

    // I can redeclare the variable which is called shadowing
    // Shadowing can use to change the type of the variable but mut can not
    let _x = x + 1;

    // const can not be mutable and it requires type too
    const _Y : u32 = 2;

    // the tuple type 
    let tup : (i8, u8 , f64) = (127, 255, 234.12);

    let (a, b, c) = tup;

    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("{a} = {tup0}, {b} = {tup1} , {c} = {tup2} ");

    // The array type
    let q = [0; 5];
    let p = [0, 0, 0, 0, 0];
    let r : [i32; 5] = [1,2,3,4,5];

    println!("{:?} = {:?} and {:?}", q, p, r);

    println!("Hello, world!");

    loop_under_loop(10);

}


fn loop_under_loop (param : u32) {

    let mut x = 0;

    'up : loop {
        println!("x = {x}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if x == 2 {
                break 'up; // This will break the 'up loop
            }
            remaining -= 1;
        }

        x += 1;
    }
    println!("last x = {x} and param is {param}");
    
}