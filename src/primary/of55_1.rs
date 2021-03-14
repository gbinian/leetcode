// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use super::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(n) => {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();
            max(max_depth(left), max_depth(right)) + 1
        },
        None => 0
    }
}

#[test]
fn test_max_depth() {
    let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    assert_eq!(max_depth(t1),1);
}
