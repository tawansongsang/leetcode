use std::collections::VecDeque;

struct MyStack {
    stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyStack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop_back().unwrap()
    }

    fn top(&self) -> i32 {
        *self.stack.get(self.stack.len() - 1).unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut my_stack = MyStack::new();
        my_stack.push(1);
        my_stack.push(2);
        let top = my_stack.top();
        assert_eq!(top, 2);
        let pop = my_stack.pop();
        assert_eq!(pop, 2);
        let is_empty = my_stack.empty();
        assert_eq!(is_empty, false);
    }
}
