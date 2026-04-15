use std::collections::{HashMap, HashSet};

use crate::leetcode::max;

pub fn sub_main() {
    //println!("from day2 sub_main")
    //test_3();
    test_76();
  // test_76_v2();
    // test_statistic_count();
}

fn test_76() {
    // println!(
    //     "{}",
    //     minimum_window_substring_76_v2("ADOBECODEBANC".to_string(), "ABC".to_string())
    // );

    let mut s = "ADOBECODEBANC";
    let mut t = "ABC";
    println!(
        "{},中包含 {} 的最小子串是 {}",
        s,
        t,
        minimum_window_substring_76_v2(s.to_string(), t.to_string())
    );

    // s = "a";
    // t = "a";
    // println!(
    //     "{},中包含 {} 的最小子串是 {}",
    //     s,
    //     t,
    //     minimum_window_substring_76(s.to_string(), t.to_string())
    // );

    // s = "a";
    // t = "aa";
    // println!(
    //     "{},中包含 {} 的最小子串是 {}",
    //     s,
    //     t,
    //     minimum_window_substring_76(s.to_string(), t.to_string())
    // );

    // s = "a";
    // t = "b";

    // s = "bbaac";
    // t = "aba";
    // println!(
    //     "{},中包含 {} 的最小子串是 {}",
    //     s,
    //     t,
    //     minimum_window_substring_76(s.to_string(), t.to_string())
    // );
}

fn test_statistic_count() {
    let cs = "ABC";

    let bs = "ADOBECODEBANC";

    let mut cmap = HashMap::new();
    let mut bmap = HashMap::new();

    for ch in cs.chars() {
        *cmap.entry(ch).or_insert(0) += 1;
    }
    for ch in bs.chars() {
        *bmap.entry(ch).or_insert(0) += 1;
    }

    println!("a count {:?}", cmap.get(&'A'));
    println!("a count {:?}", bmap.get(&'A'));
}

/**
 * 76 题 的 是短包含子串
 * s: 源串，
 * t: 目标串
 * 从s 中找到 包含t 所有字符的最小子串，   重复的算2次
 * 这个有点像下面的 第 3 题 的最长不重复子串
 * 可以老老实实的暴力解一下， 从0位置 开始， 试探，以它为开始的 最短子串。
 * 当然如果 字条不被包含， 那么就可以直接下移。 最终先出一个最短
 *
 *
 * 优化的解法中， 使用的是滑动窗口
 *
 * 这么写功能上满足， 但是时间  和空间上不满足， 265 个案例测试通过 ，共 268 全案例
 */
fn minimum_window_substring_76(s: String, t: String) -> String {
    let mut result = "".to_string();

    let slen = s.len();
    if s.len() < t.len() || t.len() == 0 {
        return result;
    }

    // 用一hashMap， 存t 的各字符个数， 并 clone着使用
    let mut t_char_count_map: HashMap<char, u32> = HashMap::new();
    for ch in t.chars() {
        *t_char_count_map.entry(ch).or_insert(0) += 1;
    }
    // 从某个位置开始， 最短能覆盖的子串的长度， 开始就没有的，
    let mut over_len_arr = vec![0; slen];
    let mut i = 0;
    let schars: Vec<char> = s.chars().collect();

    // 逐个的 字符开头 来算，  它当前字符为开始 ， 最短的覆盖
    while i < slen {
        // 当前的字符就不在  目标字符串中， 直接不管，从下一个开始
        if !t_char_count_map.contains_key(&schars[i]) {
            i += 1;
            continue;
        }

        let mut j = i;
        let mut cur_s_char_count_map: HashMap<char, u32> = HashMap::new();

        while j < slen {
            // 如果 右指针 位置 ， 在目标字符串中， 对应左指针位置 长度+1 ， 并判断一下 是否满足覆盖， 满足就直接退出 这个左指针的循环， 左指针右移
            over_len_arr[i] += 1;
            if t_char_count_map.contains_key(&schars[j]) {
                *cur_s_char_count_map.entry(schars[j]).or_insert(0) += 1;

                if is_over(&cur_s_char_count_map, &t_char_count_map) {
                    break;
                } else {
                    // 没满足覆盖 ， 要判断一一是不是 到尾部了， 到了就说明找不到， 后面的也不用再找了, 在这个内循环里， 至少是 1的， 这里强制放 0 是一种约定的通知
                    if j == slen - 1 {
                        over_len_arr[i] = 0;
                        break;
                    }
                }
            } else {
                // 没满足覆盖 ， 要判断一一是不是 到尾部了， 到了就说明找不到， 后面的也不用再找了, 在这个内循环里， 至少是 1的， 这里强制放 0 是一种约定的通知
                if j == slen - 1 {
                    over_len_arr[i] = 0;
                    break;
                }
            }

            // 如果 右指针 位置 ， 不在目标字符串中， 继续下移， 什么都不做的
            j += 1;
        }

        // 这里的  over_len_arr[i] = 0  ,是由于匹配到第i个元素 ， 由于匹配到最后都得不到覆盖， 而导致的 0， 要跳出外层循环
        if over_len_arr[i] == 0 {
            break;
        }
        i += 1;
    }

    // 寻找最短， 且不是 0 的长度  over_len_arr
    let mut min = slen + 1;
    let mut left = 0;
    for i in 0..over_len_arr.len() {
        if over_len_arr[i] != 0 && over_len_arr[i] < min {
            min = over_len_arr[i];
            left = i;
        }
    }

    if min != 0 && min != slen + 1 {
        let slice = &schars[left..left + min];
        result = slice.iter().collect();
    }

    result
}

/**
 * s 中指定的字符个数是否都大于等于t中的
 */
fn is_over(s: &HashMap<char, u32>, t: &HashMap<char, u32>) -> bool {
    for (ch, count) in t {
        if let Some(s_count) = s.get(&ch) {
            if *s_count < *count {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn test_76_v2(){
    let s = "ADOBECODEBANC";
    let t = "ABC";   
    minimum_window_substring_76_v2(s.to_string(),t.to_string() );
}

/**
 * 自己想的版本  能运行， 但是时间复杂度 不合要求 ， 下面是AI写的， 效率高， 也好理解 与简洁
 * 它的做法是将 
 *  string 转为byte， 
 *  所有的测试数据都是  ascii码的， ascii 就 128 个字符
 *  规定一个窗口， 左边  到 右边    一旦数量满足， 就记下， 是不是最小的窗口， 然后左侧往前移， 一直左移， 直到又不满足， 再右移去长， 
 *   也是2 个循环， 一个外层， 一个内层
 * 
 */
fn minimum_window_substring_76_v2(s: String, t: String) -> String {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();    // 转为了byte 数组, 其中的值 就是ascii码值 ， 如 t为  ABC  ， 转后就是[65,66,67]   , 这样就好办了， 
    
    // 一个ascii 字符就是  一个u8吗
   //prntln!("s_bytes {:?}", s_bytes);
    //println!("t_bytes {:?}", t_bytes);
    // 定义 一 128位长的数组， 记录 t字符 每个ascii码值 出现的次数
    let mut need = vec![0;128];
    for &i in t_bytes {   // i 是一个  &u8，   要转为u8 不是解引用 ，而是  再引用， 好奇怪, 这里表示的是引用它， 以免下面的i 再次被用时，报错
        need[i as usize] +=1;      // &u8 转为usize
    }
    let need_len =  need.iter().filter(|&&c| c> 0 ).count();  // 数组中的元素居然是 && i32， 为什么呢， need 数组明明是  Vec<i32> 的， 怎么filter 后就变为了 &&i32
    let mut formed =0; // 记录窗口中满足的数量
    let mut min_len = s.len() +1;  // 最小窗口的大小
    let mut min_left = 0;  // 最小窗口的起始位置 

    let mut left = 0;  // 从 0开始走
    let mut window = vec![0;128];
    // while left < s.len() {

    //     // 

    //     left +=1;
    // }

    for (right, &ch) in s_bytes.iter().enumerate() {
        let idx = ch as usize;   //  ch 是 &u8, 但同样的   &  放这， 没有用， 但放 上行括号中会有用
        window[idx] +=1;

        if need[idx] > 0 && window[idx] == need[idx] {
            formed +=1;
        }

        if formed == need_len && left <=right {
            let cur_len = right - left +1;
            if cur_len < min_len {
                min_len = cur_len;
                min_left = left;
            }

            // 左移 的字符
            let left_idx = s_bytes[left] as usize;
            window[left_idx] -=1;
            if need[left_idx] > 0 && window[left_idx] < need[left_idx] {
                formed -=1;
            }

            left +=1;
        }


    }



    if min_len == s.len() +1 {
        return String::new();    
    }


    s[min_left..min_left + min_len].to_string()  // s 作为string， 可以直接当作数组来用。
    

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
