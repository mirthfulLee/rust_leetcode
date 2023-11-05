//102. 二叉树的层序遍历

// Definition for a binary tree node.
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
struct Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if root == None {
            return result;
        }
        q.push_back(root.unwrap().clone());
        while !q.is_empty() {
            let cnt = q.len();
            let mut cur_layer: Vec<i32> = Vec::new();
            for _ in 0..cnt {
                let node = q.pop_front().unwrap().borrow();
                cur_layer.push(node.val);
                if let Some(kid) = node.left.clone() {
                    q.push_back(kid.clone());
                }
                if let Some(kid) = node.right.clone() {
                    q.push_back(kid.clone());
                }
            }
            result.push(cur_layer);
        }
        result
    }
}

fn main() {
    print!("hello");
}
