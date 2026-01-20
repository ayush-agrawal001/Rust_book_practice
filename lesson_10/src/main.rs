
fn largest<T : PartialOrd>( arr1 : &[T] ) -> &T { // Partial order is a trait implimented in T genric

    let mut largest = &arr1[0];

    for x in arr1 {
        if x > largest {
            largest = x;
        };
    };

    largest
}

fn longest<'a>(str1 : &'a str, str2 : &'a str) -> &'a str {
    if str1.len() > str2.len() {str1} else {str2}
}


fn main() {

    let array_1 = vec![0, 300, 2, 14,534, 3,234, 4];

    let largest = largest(&array_1);

    println!("{}", largest);


    // Lifetimes
    // if 'a is the lifetime where the var is defined and 'b is the lifetime of the value is given to the var
    // Then 'b > 'a

    let str1 = "Ayush";
    let str2 = "Agrawal";

    let str3 = longest(str1, str2);

    println!("{str3}");

}
