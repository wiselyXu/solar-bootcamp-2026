
// 本题目是一个算法题 ，   删除链表的倒数第 N 个结点（LeetCode 19 Medium）

#[derive(PartialEq, Eq, Clone,Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,

}