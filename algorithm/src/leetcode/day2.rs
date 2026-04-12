use std::collections::{HashMap, HashSet};

use crate::leetcode::max;

pub fn sub_main() {
    //println!("from day2 sub_main")
    test_3();
}

// 最长子串：
// 能第一想到的就是 2 层循环的那种， 但不会是n*n 复杂度，  因为会剪枝，
// 定义  max_len = 0;  max_from = 0;   len - i <= max_len 直接跳出
// 要有一个hash map 装一下， 各字段所在的下标， 虽然感觉麻烦点， 但是可以做
fn length_of_longest_substring_3(s: String) -> i32 {
    let len = s.len();
    if len == 0 {
        return 0;
    }
    if len == 1 {
        return 1;
    }
    //let chars = s.as_mut_vec();

    let mut max_len = 0;

    //let mut charIndexMap: HashMap<char, HashSet<usize>> = HashMap::new();
    // for i in 0..len {
    //     if len -i <= max_len {
    //         break;
    //     }
    //     let mut set = HashSet::new();
    //     for j in (i+1)..len {

    //         if (set.contains(chars[j])){

    //         }
    //     }
    // }

    // 不知道如何将字符串转为  数组， 所以 还是用  滑动窗口的办法 ， 这样更节省时间， 只是  不要 剪枝
    let mut last_char_index_map = HashMap::new();
    let mut left = 0;
    for (right, ch) in s.chars().enumerate() {
        if let Some(&prev) = last_char_index_map.get(&ch) {
            if prev >= left {
                left = prev + 1;
            }
        }

        last_char_index_map.insert(ch, right);

        max_len = max_len.max(right - left + 1); // 每一个位置 都算， 看一下能否减少呢。
    }

    // max_len = max_len.max(len - left );

    //  let mut last_pos = HashMap::new();
    //  for (right, ch) in s.chars().enumerate() {
    //     // 如果当前字符已经出现过，且其位置在窗口内，则移动左边界到该字符的下一个位置
    //     if let Some(&prev) = last_pos.get(&ch) {
    //         if prev >= left {
    //             left = prev + 1;
    //         }
    //     }
    //     // 更新当前字符的最新位置
    //     last_pos.insert(ch, right);
    //     // 更新最大长度（窗口长度 = right - left + 1）
    //     max_len = max_len.max(right - left + 1);
    // }

    max_len as i32
}

/**
 * 思想也是滑动窗口思想， 但做了2点优化， 所以使用上 稍麻烦些
 * 1， 剪枝：  如果左侧开始的剩余长度  <= max_len 就退出，后面不可能出现更大的 的长度了
 * 2， 计算max_len的时机， 就是left 往前移的的时候， 这里直接right -left  + 1  这时会缩小
 */
fn length_of_longest_substring_3_v2(s: String) -> i32 {
    let len = s.len();
    if len == 0 {
        return 0;
    }
    if len == 1 {
        return 1;
    }

    let mut max_len = 0;

    let chars: Vec<char> = s.chars().collect();
    let mut last_index_map = HashMap::new();
    let mut left = 0;
    let mut break_left = false;
    for i in 0..len {
        if len - left <= max_len {
            break_left = true;
            break;
        }
        let ch = chars[i];

        // 这个长度有问题吗？
        if let Some(&prev) = last_index_map.get(&ch) {
            
            if prev >= left {
                max_len = max_len.max(i - 1 - left + 1);
                left = prev + 1;
                
            }
        }

        last_index_map.insert(ch, i);
    }

    if !break_left {
        max_len = max_len.max(len - left);
    }

    max_len as i32
}

fn test_3() {
    let s = "abcabcbb".to_string();
    let len = length_of_longest_substring_3_v2(s);
    println!("最长子串长度: {}", len);

    // assert_eq!(length_of_longest_substring_3_v2("abcabcbb".to_string()), 3);
    // assert_eq!(length_of_longest_substring_3_v2("bbbbb".to_string()), 1);
    // assert_eq!(length_of_longest_substring_3_v2("pwwkew".to_string()), 3);
    // assert_eq!(length_of_longest_substring_3_v2("".to_string()), 0);
    // assert_eq!(length_of_longest_substring_3_v2("au".to_string()), 2);
    // assert_eq!(length_of_longest_substring_3_v2("abba".to_string()), 2);
    // assert_eq!(length_of_longest_substring_3_v2("dvdf".to_string()), 3);
    // // 包含 Unicode 字符
    // assert_eq!(length_of_longest_substring_3_v2("你好世界世好".to_string()), 4);
    //   assert_eq!(length_of_longest_substring_3("ab👨‍👩‍👧‍👦cd".to_string()), 6);
}
