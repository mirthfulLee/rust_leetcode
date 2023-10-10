// 98. 验证二叉搜索树
struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::bst_checker(root, None, None)
    }
    pub fn bst_checker(
        root: Option<Rc<RefCell<TreeNode>>>,
        floor: Option<i32>,
        ceiling: Option<i32>,
    ) -> bool {
        match root {
            None => true,
            Some(node) => {
                let val = node.borrow().val;
                floor.map_or(true, |x| x < val)
                    && ceiling.map_or(true, |x| x > val)
                    && Solution::bst_checker(node.borrow().left.clone(), floor, Some(val))
                    && Solution::bst_checker(node.borrow().right.clone(), Some(val), ceiling)
            }
        }
    }
}
fn main() {}
