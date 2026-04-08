// 与原版本比， 这里使用 VecDeque 作为key_list， 因为它支持在两端高效地添加和删除元素， 适合实现 LRU 缓存的功能， 同时将key_list 改名为order
// 因为Vecqueu 的retain 是稳定的
use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    cache: HashMap<i32, i32>,
    capacity: i32,
    order: VecDeque<i32>, // 最近使用的在尾部，最久未用的在头部
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            cache: HashMap::new(),
            capacity,
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&value) = self.cache.get(&key) {
            // 将 key 移动到 VecDeque 的尾部（表示最近使用）
            self.order.retain(|&k| k != key); // 移除旧位置
            self.order.push_back(key); // 添加到尾部
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            // 更新现有键的值
            self.cache.insert(key, value);
            // 将 key 移动到 VecDeque 的尾部（表示最近使用）
            self.order.retain(|&k| k != key); // 移除旧位置
            self.order.push_back(key); // 添加到尾部
        } else {
            if self.cache.len() as i32 >= self.capacity {
                // 移除最久未使用的键
                if let Some(old_key) = self.order.pop_front() {
                    self.cache.remove(&old_key);
                }
            }
            // 添加新键值对
            self.cache.insert(key, value);
            self.order.push_back(key); // 添加到尾部
        }
    }
}   