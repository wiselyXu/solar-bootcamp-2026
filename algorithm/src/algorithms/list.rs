// 本题目是一个算法题 ，   删除链表的倒数第 N 个结点（LeetCode 19 Medium）

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline] // 为什么要加  inline?  
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//     let mut dummy = Some(Box::new(ListNode::new(0)));0
//     dummy.as_mut().unwrap().next = head;

//     let mut fast = dummy.as_ref().cloned();
//     let mut slow = dummy.as_mut();

//     // fast 先走n 步
//     let mut steps = n;
//     while steps > 0 {
//         if let Some(node) = fast {
//             fast = node.next;
//         }

//         steps -= 1;
//     }

//     // fast 和slow 一起走， 直到fast 到尾部
//     while fast.is_some() {
//         if let (Some(f), Some(s)) = (fast, slow.as_mut()) {
//             fast = f.next;
//             slow = s.next.as_mut();
//         }
//     }

//     // slow 现在就指向了要删除的节点
//     if let Some(s) = slow {
//         s.next = s.next.as_ref().and_then(|node| node.next.clone());
//     }

//     dummy.unwrap().next
// }

// pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//     let mut dummy = Box::new(ListNode { val: 0, next: head });
//     let mut fast = &dummy;
//     let mut slow = &mut dummy;

//     for _ in 0..n {
//         if let Some(node) = fast.next.as_ref() {
//             fast = node;
//         }
//     }

//     while fast.next.is_some() {
//         fast = fast.next.as_ref().unwrap();
//         slow = slow.next.as_mut().unwrap();
//     }

//     slow.next = slow.next.as_mut().unwrap().next.take();
//     dummy.next
// }

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {next: head, val:0});
    // 第一趟： 计算链表长度（用不可变借用， 避免冲突）
    let mut length = 0;
    let mut cur = &dummy;
    while cur.next.is_some() {
        length +=1;
        cur = cur.next.as_ref().unwrap();
    }

    // 如果 n >lenght, 直接返回  
    if n > length {
        return dummy.next;
    }

    // 第二趟，走到要删除的节点的前一个 （用可变借用）, 将指针指向它下一节点的下一节点。
    let mut cur = &mut dummy;

    for _ in 0..(length -n ) { // 走lenght -n 步， 到prev
        cur = cur.next.as_mut().unwrap();
    }

    cur.next = cur.next.as_mut().unwrap().next.take();
    dummy.next



}


pub fn vec_to_list_node(arr: Vec<i32>)  -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }
    
    
    let mut head = Some(Box::new(ListNode::new(arr[0] )));
    let mut tail = head.as_mut();
    for &val in &arr[1..] {

        let new_node = Some(Box::new(ListNode::new(val )));

        if let Some(node)  = tail.as_mut() {
            node.next = new_node;
        }
        
        // tail 移动到新节点的option 位置
      //  tail = tail.unwrap().next.as_mut();
        {
            tail = tail.and_then(|node| node.next.as_mut());
        }
    }

    head
    // let mut head = Some(Box::new(ListNode::new(arr[arr.len() -1] )));
    // for &val in arr.iter().rev().skip(1) {
    //     let mut new_node = Box::new(ListNode::new(val));
    //     new_node.next = head;
    //     head = Some(new_node);
    // }

    // head
}


pub fn print_list_node(head: Option<Box<ListNode>>) {
    if head.is_none() {
        println!("an empty listNode");
    }

    let mut cur = head;
    
    while true {
        if let Some(node)  = &cur {
                print!("{}  ", node.val);
                cur = cur.unwrap().next;
        }else {
            break;
        }
    }
     
    

}