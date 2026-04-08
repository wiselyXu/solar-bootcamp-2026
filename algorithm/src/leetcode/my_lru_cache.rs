pub struct LRUCache {
    cache: std::collections::HashMap<i32, i32>,
    capacity: i32,
    key_list: std::collections::LinkedList<i32>, // 最近使用的在尾部，最久未用的在头部
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            cache: std::collections::HashMap::new(),
            capacity,
            key_list: std::collections::LinkedList::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        // ← 必须改成 &mut self
        if let Some(&value) = self.cache.get(&key) {
            // 用 &value 拿到 i32
            // 将 key 移动到链表尾部（表示最近使用）
            // self.key_list.retain(|k| *k != key);       // ← 正确写法
            // 方式1， 直接过滤掉 key， 这个代码有点大，效率不高, 每取一下， 就要复制， 并删除， 动作太大了
            self.key_list = self
                .key_list
                .clone()
                .into_iter()
                .filter(|&k| k != key)
                .collect();

            // 方式2， 直接找到 key 的位置并删除，效率更高， 但需要遍历链表，效率也不高， cursor_front_mut() 是一个unstable API， 可能会在未来的 Rust 版本中发生变化，所以需要谨慎使用
            // let mut cursor = self.key_list.cursor_front_mut();
            // while let Some(&k) = cursor.current() {
            //     if k == key {
            //         cursor.remove_current(); // 删除当前元素
            //         break;
            //     }
            //     cursor.move_next(); // 移动到下一个元素
            // }

            // 方式3， 对方式1的修正，使用mem::take， 即从原来的位置取出链表， 进行过滤后再放回去， 这样就避免了 clone 的开销，改后后又挂回去
            let mut list = std::mem::take(&mut self.key_list);
            list = list.into_iter().filter(|&k| k != key).collect();
            // 需要这样显示的把过滤后的链表放回去， 因为 list 是一个局部变量， 作用域结束后会被销毁， 需要把它放回到 self.key_list 中
            self.key_list = list;

            self.key_list.push_back(key);

            Some(value)
        } else {
            None
        }
    }

    // put 方法后面也要改成 &mut self，这里先只改 get
    pub fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            // 更新现有键的值
            self.cache.insert(key, value);
            // 将 key 移动到链表尾部（表示最近使用）
            let mut list = std::mem::take(&mut self.key_list);
            list = list.into_iter().filter(|&k| k != key).collect();
            // 需要这样显示的把过滤后的链表放回去， 因为 list 是一个局部变量， 作用域结束后会被销毁， 需要把它放回到 self.key_list 中
            self.key_list = list;

            self.key_list.push_back(key);
        } else {
            // 插入新键值对
            if self.cache.len() as i32 == self.capacity {
                // 删除最久未使用的键值对
                if let Some(old_key) = self.key_list.pop_front() {
                    self.cache.remove(&old_key);
                }
            }
            self.cache.insert(key, value);
            self.key_list.push_back(key);
        }
    }
}
