// 449. 序列化和反序列化二叉搜索树
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
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Codec {
    fn new() -> Self {
        return Codec {};
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "".to_string(),
            Some(node) => {
                let node = node.borrow();
                format!(
                    "{},{}{}",
                    node.val,
                    self.serialize(node.left.clone()),
                    self.serialize(node.right.clone())
                )
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let pre_order = data
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Codec::build_node(&pre_order, 0, pre_order.len() - 1)
    }

    fn build_node(
        pre_order: &Vec<i32>,
        left: usize,
        right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        // left: [left+1, partition), right: [partition, right]
        let mut partition = left;
        while partition <= right && pre_order[left] >= pre_order[partition] {
            partition += 1;
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: pre_order[left],
            left: Codec::build_node(pre_order, left + 1, partition - 1),
            right: Codec::build_node(pre_order, partition, right),
        })))
    }
}
