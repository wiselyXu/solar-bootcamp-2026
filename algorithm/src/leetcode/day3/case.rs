pub fn sub_main() {
    println!("come from the leetcode/day3/case  sub main");
}

// Definition for singly-linked list.
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

pub struct Solution;

impl Solution {
    /**
     * 定义一个临时节点 ， 专门暂存 当前节点的next， 直接到当前节点 的next 为空
     */
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        // next 暂存 当前节点的next, take 掉所有权
        // node的 next 就为 prev （初始为none
        // prev 转存 为当前
        // 当前就 转存为next
        // 一般不会说 node ==None 说法
        while let Some(mut node) = curr {
            let temp = node.next.take();
            node.next = prev;
            prev = Some(node); //为什么要Some， 这是一个Option的 
            curr = temp;
        }
        prev
    }

    /**
     * leetcode  第 26 题
     * 合并 2 个有序列表 ， 这个还是比较简单的
     * 定义一个新节点
     */
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy; // 作为可变引用 ， 后面修改它的next

        // 定2个当前节点
        let mut l1 = list1;
        let mut l2 = list2;
        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            // 为什么要 as_ref ，不用是所有权都给过去的， 而这是一个循环。

            // 用一临时变量temp ， 记下 小链的下一节点， 后面要继续用的，
            // 还像上 一题 的交换
            // tail.next = l1.take()
            // l1 = temp
            if node1.val < node2.val {
                let temp = l1.as_mut().unwrap().next.take(); //可变， 并直接给一下个的所有权拿出来。   这样 temp 实现就是余下节点的头了
                tail.next = l1.take(); // take 最前的节点 ， 
                tail = tail.next.as_mut().unwrap(); // tail 本身再往下移
                l1 = temp;
            } else {
                let temp = l2.as_mut().unwrap().next.take();
                tail.next = l2.take(); // take 最前的节点 ， 
                tail = tail.next.as_mut().unwrap(); // tail 本身再往下移
                l2 = temp;
            }
        }

        tail.next = l1.or(l2);
        dummy.next
    }


    /**
     * leetcode 19   删除单链表的倒数 第几个节点
     * 这个定义两个指针 p1,p2 ， p1 一直走到尾， 数完这个链共几个节点， 如 n 个 ,  现在删除 倒数第k 个
     * 则第二个指针  走 n-k +1  就是要删除的点，  这样走到 n-k +2 时， 就要开始处理， 它的next  = next .next 即可
     * 
     * 这里要如何重新定义 
     */
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{val:0,next : head});
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();  // as_mut 后， 不是一个  mut Box   而是里面的具体类型 mut

        // 由于是倒数第几个， 所以它应该是， 快指针， 先走n， 然后慢指针与快指针同时走， 最终快指针到达 尾时，  慢指针就在倒数第n 位
        for _ in 0..n{
            fast = fast.next.unwrap();    // 有可能 这个n 比链的长度还长， 怎么办， 可能 这里就不考虑了
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        // 此时slow 指向要删除的节点的前一个节点
        let to_remove = slow.next.take();
        slow.next = to_remove.and_then(|node| node.next);  // 如果后面没有了， 会怎样？

        dummy.next
        
    }
}

fn test_reverse_list() {}
