use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
// 纯后写  链表结构 ， 实现LRU， 用的是AI生成的代码 ， 手抄起来有点累呀。而且有点抄不懂
type Link = Option<Rc<RefCell<Node>>>;
//type Link = Option<Node>;

struct Node {
    key: i32,
    value: i32,
    prev: Link, // 不像java 样可以写为NODE，   rust 会认为是无限长度  infinite size
    next: Link,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>, // 不是我想的直接Node
    head: Link,                           //不用node
    tail: Link,
}

impl LRUCache {
    pub fn new(capacity: i32) -> LRUCache {
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

        head.borrow_mut().next = Some(Rc::clone(&head));
        tail.borrow_mut().prev = Some(Rc::clone(&tail));


        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: Some(head),
            tail: Some(tail),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node_rc = Rc::clone(node);
            self.move_to_tail(&node_rc);
            node_rc.borrow().value // 明明 .value 出不来的， 但没报错
        } else {
            -1
        }
    }

    // 如果存在 ，先设置新值，再 像get 一样  remove_to_tail
    // 如不存在， 则直接新建node， add_to_tail
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(exist_node) = self.map.get(&key) {
            //exist_node.borrow_mut().value = value;
            let node_rc = Rc::clone(exist_node);
            {
                let mut n = node_rc.borrow_mut();
                n.value = value;
            }
            self.move_to_tail(&node_rc);
        } else {
            if self.map.len() >= self.capacity {
                self.evict_node();
            }

            let new_node = Rc::new(RefCell::new(Node {
                key,
                value,
                prev: None,
                next: None,
            }));

            self.map.insert(key, Rc::clone(&new_node));
            self.add_to_tail(&new_node);
        }
    }
    // ---------------------  私有方法  ---------------

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        self.remove_node(node);
        self.add_to_tail(node);
    }

    // 将节点移到队尾， 表示最新活动的， 新加的直接用这个 就可以了， 但如果是老的， 需要调本方法前先删除
    fn add_to_tail_v2(&mut self, node: &Rc<RefCell<Node>>) {
        //-- 检查
        // if let Some(last) = &self.head.as_ref().unwrap().borrow().next {
        //     if last.borrow().key != -1 {
        //         println!("已有非head的前节点， 它的key {}",last.borrow().key);
        //     }else {
        //         println!("还处于初始,它的key {}",last.borrow().key);
        //     }
        // }
        //----
        // let prev = tail.borrow().prev.as_ref().unwrap().borrow().prev;

        //let tail = self.tail.as_ref().unwrap(); //.cloned();  // 为什么要clone呢， clone 出来 好像是浅拷贝，不是深的
        // let tail = self.tail.as_ref().cloned();
        // let prev = tail.unwrap().borrow().prev.clone();

        // prev.borrow_mut().next = Some(Rc::clone(node));
        // tail.borrow_mut().prev = Some(Rc::clone(node));

        // node.borrow_mut().prev = Some(Rc::clone(&prev));
        // node.borrow_mut().next = Some(Rc::clone(tail));

        let tail = self.tail.as_ref().unwrap().clone(); // Rc 克隆（引用计数+1）
        let prev = tail.borrow().prev.as_ref().unwrap().clone(); // prev 应该是当前最后一个节点（或 head）

        // 1. 把新节点插入到 prev 和 tail 之间
        prev.borrow_mut().next = Some(Rc::clone(node));
        tail.borrow_mut().prev = Some(Rc::clone(node));

        // 2. 设置新节点的 prev 和 next
        node.borrow_mut().prev = Some(Rc::clone(&prev));
        node.borrow_mut().next = Some(Rc::clone(&tail));

        if let Some(last) = &self.head.as_ref().unwrap().borrow().next {
            if last.borrow().key != -1 {
                println!("已插入节点， 它的key {}", last.borrow().key);
            } else {
                println!("仍然指向了 尾节点")
            }
        }

        if let Some(first) = &self.tail.as_ref().unwrap().borrow().prev {
            if first.borrow().key != -1 {
                println!("已插入节点， 它的key {}", first.borrow().key);
            } else {
                println!("仍然指向了 头节点")
            }
        }
    }

    fn add_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let tail = self.tail.as_ref().unwrap().clone();
        let prev = {
            let tail_borrow = tail.borrow();
            tail_borrow.prev.as_ref().unwrap().clone()
        };

        // 插入新节点
        prev.borrow_mut().next = Some(Rc::clone(node));
        tail.borrow_mut().prev = Some(Rc::clone(node));

        // 设置新节点的前后指针
        node.borrow_mut().prev = Some(Rc::clone(&prev));
        node.borrow_mut().next = Some(Rc::clone(&tail));

        ////
        ///
        ///
        if let Some(last) = &self.head.as_ref().unwrap().borrow().next {
            if last.borrow().key != -1 {
                println!("已插入节点， 它的key {}", last.borrow().key);
            } else {
                println!("仍然指向了 尾节点")
            }
        }

        if let Some(first) = &self.tail.as_ref().unwrap().borrow().prev {
            if first.borrow().key != -1 {
                println!("已插入节点， 它的key {}", first.borrow().key);
            } else {
                println!("仍然指向了 头节点")
            }
        }
    }

    // 删除最久没有用的节点， 即头部第一个真实节点
    fn evict_node(&mut self) {
        let node = &self.head;

        if let Some(head) = &self.head {
            //let Some(_node) = self.head.unwrap().borrow().next ;

            if let Some(first) = head.borrow().next.clone() {
                if first.borrow().key != -1 {
                    let key = first.borrow().key;
                    self.remove_node(&first);
                    self.map.remove(&key);
                }
            }
        }
    }

    fn remove_node(&self, node: &Rc<RefCell<Node>>) {
        // 这个节点的 前 节点 拿出来，   后节点也拿出来， 由于代码的控制， 前后节点都不可能为空， 即不会将两个哨兵节点传过来
        let prev = node.borrow_mut().prev.as_ref().unwrap().clone();
        let next = node.borrow_mut().next.as_ref().unwrap().clone();

        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));
    }
}
