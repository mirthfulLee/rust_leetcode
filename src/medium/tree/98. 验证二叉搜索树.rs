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
        if let None = root {
            return true;
        }
        Solution::sub_bst_info(root).0
    }
    // (valid, min_val, max_val)
    pub fn sub_bst_info(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
        let mut valid_flag = true;
        let mut min_val = i32::MAX;
        let mut max_val = i32::MIN;
        if let Some(node) = root {
            min_val = node.borrow().val;
            max_val = node.borrow().val;
            if let Some(left_node) = &node.borrow().left {
                valid_flag &= node.borrow().val > left_node.borrow().val;
                if valid_flag {
                    let (valid, sub_min, sub_max) = Solution::sub_bst_info(Some(left_node.clone()));
                    valid_flag = valid_flag && valid && sub_max < node.borrow().val;
                    if sub_min < min_val {
                        min_val = sub_min;
                    }
                }
            }
            if valid_flag {
                if let Some(right_node) = &node.borrow().right {
                    valid_flag &= node.borrow().val < right_node.borrow().val;
                    if valid_flag {
                        let (valid, sub_min, sub_max) =
                            Solution::sub_bst_info(Some(right_node.clone()));
                        valid_flag = valid_flag && valid && sub_min > node.borrow().val;
                        if sub_max > max_val {
                            max_val = sub_max;
                        }
                    }
                }
            }
        }
        (valid_flag, min_val, max_val)
    }
}
fn main() {}
