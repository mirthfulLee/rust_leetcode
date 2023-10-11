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
#[derive(Default)]
struct Codec {
    cur_idx: usize,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Codec {
    fn new() -> Self {
        return Codec::default();
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

    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let pre_order = data
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        self.cur_idx = 0;
        self.build_node(&pre_order, None, None)
    }

    fn build_node(
        &mut self,
        pre_order: &Vec<i32>,
        floor: Option<i32>,
        ceiling: Option<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if self.cur_idx >= pre_order.len() {
            return None;
        }
        let val = pre_order[self.cur_idx];
        if let Some(min) = floor {
            if val < min {
                return None;
            }
        }
        if let Some(max) = ceiling {
            if val > max {
                return None;
            }
        }
        self.cur_idx += 1;
        Some(Rc::new(RefCell::new(TreeNode {
            val: val,
            left: self.build_node(pre_order, floor, Some(val)),
            right: self.build_node(pre_order, Some(val), ceiling),
        })))
    }
}
