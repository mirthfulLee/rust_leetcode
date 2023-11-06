// 203. 移除链表元素

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;


impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                if val == node.val {
                    Self::remove_elements(node.next, val)
                }else {
                    node.next = Self::remove_elements(node.next, val);
                    Some(node)
                }
            },
            None => None
        }
    }
}