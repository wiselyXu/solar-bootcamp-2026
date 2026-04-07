use crate::leetcode::lru_cache::cache_trait;


pub struct LRUCache {
    capacity: usize,
    map: std::collections::HashMap<i32, i32>,
    list: std::collections::LinkedList<i32>,
}

// 移除时， 从链表头部移除， 插入时， 从链表尾部插入， 这样链表头部就是最久未使用的key， 链表尾部是最近使用的key
// 所以get 时， 要完成 remove  + add 操作
// put 时， 如为老元素， remove +add, 
            //如为新元素， 先判断是否满了， 满了就remove, 然后add
// 所以 remove 和 add 是基本原子操作， 而且是私有方法，补get， put方法中调用， 这样就不会有重复代码了
impl cache_trait::Cache<i32, i32> for LRUCache {
    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(value) = self.map.get(&key) {
            // 将key移动到链表的末尾，表示最近使用
         //   self.list.retain(|&x| x != key);
            self.list.push_back(key);
            Some(*value)
        } else {
            None
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            // 更新值并将key移动到链表的末尾
            self.map.insert(key, value);
           // self.list.retain(|&x| x != key);
            self.list.push_back(key);
        } else {
            if self.map.len() == self.capacity {
                // 移除最久未使用的key
                if let Some(old_key) = self.list.pop_front() {
                    self.map.remove(&old_key);
                }
            }
            // 插入新key和value，并将key添加到链表的末尾
            self.map.insert(key, value);
            self.list.push_back(key);
        }
    }
}