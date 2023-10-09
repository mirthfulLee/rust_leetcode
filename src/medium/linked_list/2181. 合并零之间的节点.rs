// 2181. 合并零之间的节点
struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = ListNode::new(0);
        let mut ptr = &mut new_head.next;
        let mut cur_sum = 0;
        while let Some(cur_node) = head.take() {
            if cur_node.val == 0 && cur_sum > 0 {
                *ptr = Some(Box::new(ListNode::new(cur_sum)));
                cur_sum = 0;
                ptr = &mut ptr.as_mut().unwrap().next;
            } else {
                cur_sum += cur_node.val;
            }
            head = cur_node.next;
        }
        new_head.next.take()
    }
}
fn main() {}
