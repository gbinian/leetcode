use std::rc::Rc;
use std::cell::RefCell;

/// Rc引用计数 RefCell获取内部可变性(可以在不实现Clone的情况下得到可变引用)
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left:None,
            right: None
        }
    }
}
