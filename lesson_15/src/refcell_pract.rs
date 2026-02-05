
// Creating a program that print error when max is hit

pub trait Messanger {
    fn send(&self, msg : &str);
}

pub struct LimitTracker <'a, T : Messanger> {
    messanger : &'a T,
    value : usize,
    max : usize,
}

impl <'a, T> LimitTracker<'a, T> where T : Messanger {
    pub fn new(max : usize, messanger : &'a T) -> LimitTracker<'a, T>{
        LimitTracker {
            messanger,
            value : 0,
            max
        }
    }

    pub fn set_value(&mut self, value : usize){
        self.value = value;

        let percantage_of_max = self.value as f64 / self.max as f64;

        if percantage_of_max > 1.0 {
            self.messanger.send("Error : You have used over the limit");
        } else if percantage_of_max >= 0.90 {
            self.messanger.send("You have used more then 90%");
        } else if percantage_of_max >= 0.75 {
            self.messanger.send("you have used more then 75%");
        }
    }
}


#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;

    struct MockMessanger {
        sent_message : RefCell<Vec<String>>,
    }

    impl MockMessanger {
        fn new()-> MockMessanger {
            MockMessanger { sent_message: RefCell::new(vec![]) }
        }
    }

    impl Messanger for MockMessanger {
        fn send(&self, msg : &str) {
            self.sent_message.borrow_mut().push(String::from(msg));
            print!("{msg}");
        }
    }

    #[test]
    fn it_sends_over_75(){
        let mockmessanger = MockMessanger::new();

        let mut limittracker = LimitTracker::new(100, &mockmessanger);

        limittracker.set_value(80);

        dbg!(mockmessanger.sent_message.borrow());
        assert_eq!(mockmessanger.sent_message.borrow().len(), 1);

    }
}

