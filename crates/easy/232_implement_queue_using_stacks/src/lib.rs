#[allow(unused)]
#[derive(Default, PartialEq, Debug)]
struct MyQueue {
    stack: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = MyQueue::new();
        assert_eq!(
            MyQueue {
                stack: std::collections::VecDeque::<i32>::new()
            },
            obj
        );
        let ret_1 = obj.push(1);
        assert_eq!((), ret_1);
        let ret_2 = obj.push(2);
        assert_eq!((), ret_2);
        let ret_3: i32 = obj.peek();
        assert_eq!(1, ret_3);
        let ret_4: i32 = obj.pop();
        assert_eq!(1, ret_4);
        let ret_5: bool = obj.empty();
        assert_eq!(false, ret_5);
    }
}
