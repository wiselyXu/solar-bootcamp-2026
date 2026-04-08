use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Link,
    next: Link,
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,  // key -> Node 指针
    head: Link,   // 哨兵头节点（最久未使用）
    tail: Link,   // 哨兵尾节点（最近使用）
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node {
            key: -1,
            value: -1,
            prev: None,
            next: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            key: -1,
            value: -1,
            prev: None,
            next: None,
        }));

        // 连接哨兵节点
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            head: Some(head),
            tail: Some(tail),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            let node_rc = Rc::clone(node);
            self.move_to_tail(&node_rc);   // O(1) 移动到最近使用位置
            node_rc.borrow().value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            // 已存在：更新值 + 移动到尾部
            let node_rc = Rc::clone(node);
            {
                let mut n = node_rc.borrow_mut();
                n.value = value;
            }
            self.move_to_tail(&node_rc);
            return;
        }

        // 新节点
        if self.cache.len() >= self.capacity {
            self.remove_lru();   // 删除最久未使用的节点
        }

        // 创建新节点并插入到尾部
        let new_node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));

        self.cache.insert(key, Rc::clone(&new_node));
        self.add_to_tail(&new_node);
    }

    // ==================== 核心 O(1) 操作 ====================

    // 把节点移动到尾部（最近使用）
    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        self.remove_node(node);
        self.add_to_tail(node);
    }

    // 从链表中删除一个节点（O(1)）
    fn remove_node(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().unwrap().clone();
        let next = node.borrow().next.as_ref().unwrap().clone();

        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));
    }

    // 在尾部添加一个节点（O(1)）
    fn add_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let tail = self.tail.as_ref().unwrap();
        let prev = tail.borrow().prev.as_ref().unwrap().clone();

        prev.borrow_mut().next = Some(Rc::clone(node));
        tail.borrow_mut().prev = Some(Rc::clone(node));

        node.borrow_mut().prev = Some(Rc::clone(&prev));
        node.borrow_mut().next = Some(Rc::clone(tail));
    }

    // 删除最久未使用的节点（头部第一个真实节点）
    fn remove_lru(&mut self) {
        let head = self.head.as_ref().unwrap().clone();  // 先 clone 哨兵头节点

        // 获取最久未使用的节点（head.next）
        let first = {
            let head_ref = head.borrow();
            head_ref.next.as_ref().cloned()              // 只在这一小块作用域内 borrow
        };
    
        if let Some(first_node) = first {
            if first_node.borrow().key == -1 {
                return; // 没有真实节点
            }
    
            let key = first_node.borrow().key;
    
            // 关键：先结束所有不可变借用，再进行可变操作
            self.remove_node(&first_node);
            self.cache.remove(&key);
        }
    }
}