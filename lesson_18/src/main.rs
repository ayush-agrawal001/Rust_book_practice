
// Objects are something that have data and the procedures that operates on the data

mod trait_object;
mod implimenting_states_for_OOPs;

use crate::block_1::AvreagedCollection;
use trait_object::trait_object_fn;
use implimenting_states_for_OOPs::states_impl_main;

mod block_1{
    pub struct AvreagedCollection {
        list : Vec<i32>,
        average : f64
    }
    impl AvreagedCollection {

        pub fn new() -> AvreagedCollection{
            AvreagedCollection {
                list : vec![],
                average : 0.0
            }
        }

        pub fn update_average(&mut self) {
            let total : i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }

        pub fn add(&mut self, value : i32 ) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(val) => {
                    self.update_average(); 
                    Some(val)
                }
                None => None
            }
        }


        pub fn average(&self) -> f64{
            self.average
        }

    }
}


fn main() {
    let mut average_struct_1 = AvreagedCollection::new();

    average_struct_1.add(10);
    average_struct_1.add(20);
    average_struct_1.add(30);
    average_struct_1.add(40);
    average_struct_1.add(50);
    average_struct_1.add(50);

    average_struct_1.remove();

    let average = average_struct_1.average();

    println!("average is {}", average);

    trait_object_fn();

    states_impl_main();
}
