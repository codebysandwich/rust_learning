/*
* File       : rust_pl/src/box_pointer/lib.rs
* Time       ：2025/2/21 11:16
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

pub trait SendMessenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + SendMessenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: SendMessenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl SendMessenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    /// **运行时错误**
    // impl SendMessenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut borrowed_one = self.sent_messages.borrow_mut();
    //         let mut borrowed_two = self.sent_messages.borrow_mut();
    // 
    //         borrowed_one.push(String::from(message));
    //         borrowed_two.push(String::from(message));
    //     }
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
