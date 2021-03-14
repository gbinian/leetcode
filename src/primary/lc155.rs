use std::collections::VecDeque;
use std::cmp::min;
/// 最小栈
/// 设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
//
// push(x) —— 将元素 x 推入栈中。
// pop() —— 删除栈顶的元素。
// top() —— 获取栈顶元素。
// getMin() —— 检索栈中的最小元素。
///MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin();   --> 返回 -3.
// minStack.pop();
// minStack.top();      --> 返回 0.
// minStack.getMin();   --> 返回 -2.
struct MinStack {
    min_stack: VecDeque<i32>,
    num_stack: VecDeque<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            min_stack: VecDeque::new(),
            num_stack: VecDeque::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.num_stack.push_back(x);
        if self.min_stack.is_empty() {
            self.min_stack.push_back(x);
        } else {
            self.min_stack.push_back(min(
                *self.min_stack.back().unwrap(),
                x
            ))
        }
    }

    fn pop(&mut self) {
        self.min_stack.pop_back();
        self.num_stack.pop_back();
    }

    fn top(&mut self) -> i32 {
        *self.num_stack.back().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.back().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[test]
fn test_min_stack() {
    let mut s = MinStack::new();
    s.push(2);
    s.push(1);
    s.push(3);
    assert_eq!(s.top(), 3);
    assert_eq!(s.get_min(), 1);
    s.pop();
    assert_eq!(s.top(), 1);
    assert_eq!(s.get_min(),1);

}
