use std::collections::HashMap;

pub fn sub_main() {
    // test_max();
    // test_two_sum_01();
    test_group_anagrams_49();
    // test_will_sort_string();
}

// 217. 存在重复元素, 可以用map 来解， 当然这里也有set ，
fn contains_duplicate_217(nums: Vec<i32>) -> bool {
    let mut set = std::collections::HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    false
}

// 217: 还有第二种解法 ，用排序， 先排序， 然后比较相邻的元素是否相等， 如果有相等的元素， 就说明存在重复元素
// sort_unstable()  相同值 的元素可能原来的相对顺序会改变 ， 它用的是快速排序算法， 这种算法的平均时间复杂度是 O(n log n)， 最坏情况是 O(n^2)， 但是在实际应用中， 它的性能通常比其他排序算法更好， 因为它具有较小的常数因子和较好的缓存性能。
//  sort() 也行的， 是稳定的排序算法， 但是它的性能可能不如 sort_unstable()， 因为它需要更多的内存来维护稳定性， 但是在某些情况下， 稳定性可能是必要的， 比如当你需要保持相同值的元素的相对顺序时， 就需要使用 sort() 来保证稳定性。
// windows(2) 这个方法会创建一个滑动窗口， 每次窗口包含两个相邻的元素， 然后 any(|w| w[0] == w[1]) 这个方法会检查每个窗口中的两个元素是否相等， 如果有任何一个窗口中的两个元素相等， 就说明存在重复元素， 返回 true，否则返回 false。
// 它就 是rust idomatically 的写法
fn contains_duplicate_217_02(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();
    nums.windows(2).any(|w| w[0] == w[1]) // 
}

fn test_group_anagrams_49() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    //  let strs = vec!["b".to_string(), "".to_string()];
    let result = group_anagrams_49_3(strs);
    println!("分组结果是: {:?}", result);
}
/**
 * 49. 字母异位词分组
 * 自己写代码
 * 1， 维护一个已使用的 下标列表，  usedIndexSet :HashSet<>,
 *       定义一个  Vec<Vec<String>> 来存储结果，
 * 2，循环每一个元素， 如果在 usedIndexSet 中， 就跳过，
 *     否则就把它加入到 usedIndexSet 中，
 *      然后循环剩下的元素， 如果它们是异位词， 就把它们加入到当前的分组中，并且把它们的下标也加入到 usedIndexSet 中
 *       判断是否为异位词的办法， 1 是长度一样， 二是  每个元素都出现的次数一样， 这个可以用一个长度为 26 的数组来统计每个字母出现的次数， 最后比较两个数组是否相等， 如果相等， 就说明它们是异位词
 *
 */
fn group_anagrams_49(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut usedIndexSet = std::collections::HashSet::new();
    let mut result = Vec::new();
    for i in 0..strs.len() {
        if usedIndexSet.contains(&i) {
            continue;
        }

        let str = &strs[i];
        let mut group = Vec::new();
        group.push(str.clone()); // str 后面还要用， 这里如果不clone， 就会move进group， 导致不能用
        let count_map = statistic_char_count(str.clone());
        for j in i + 1..strs.len() {
            if usedIndexSet.contains(&j) {
                continue;
            }
            if is_anagram_for_242(&count_map, strs[j].clone()) {
                group.push(strs[j].clone());
                usedIndexSet.insert(j);
            }
        }

        result.push(group);
    }

    result

    // let mut map = std::collections::HashMap::new();

    // for s in strs {
    //     let mut chars: Vec<char> = s.chars().collect();
    //     chars.sort_unstable();
    //     let key: String = chars.into_iter().collect();  // into_iter  不会排序吧。 这样的key 会一样吗？

    //     map.entry(key).or_insert(Vec::new()).push(s);  // insert(Vec::new()),  key 都一样的吗
    // }

    // map.into_values().collect()
}

fn group_anagrams_49_2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect(); //   into_iter  不会排序吧。 这样的key 会一样吗？, 这里的collect 是把 chars 这个 Vec<char> 收集成一个 String， 这里的 key 就是排序后的字符串， 所以相同的异位词会有相同的 key

        map.entry(key).or_insert(Vec::new()).push(s); // insert(Vec::new()),  key 都一样的吗
    }

    for (key, group) in &map {
        println!("key: {}, group: {:?}", key, group);
    }

    map.into_values().collect() // collect 这个方法会把迭代器中的元素收集到一个集合中， 这里我们把 map 中的值收集到一个 Vec<Vec<String>> 中， 因为 map 的值是 Vec<String>， 所以 collect 会把它们收集到一个 Vec<Vec<String>> 中
}

// 答案中还是觉得 只有 26 个英文字母， 用一个长度为 26 的数组来统计每个字母出现的次数， 这个解法也不错， 但是如果字符集比较大， 就不太适用了， 但是如果字符集比较小， 就很高效了
// 用一count 作为key, 会更快， 原理还是一样，所有的  anagram  都会有一样的count，这样虽然会有2个循环， 但速度 几乎  0秒
fn group_anagrams_49_3(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8; 26], Vec<String>> = std::collections::HashMap::new();

    for s in strs {
        let mut count: [u8; 26] = [0; 26];
        for c in s.chars() {
            count[c as usize - b'a' as usize] += 1 as u8;
        }
        map.entry(count).or_insert(Vec::new()).push(s);
    }

    map.into_values().collect::<Vec<Vec<String>>>()  // 它与  map.into_values().collect() 的区别是， 前者会把 map 中的值收集到一个 Vec<Vec<String>> 中， 
    //后者会把 map 中的值收集到一个 Vec<Vec<String>> 中， 但是前者会指定类型， 后者会根据上下文推断类型， 在这个例子中， 
    //前者会更明确一些， 因为我们知道我们要收集成一个 Vec<Vec<String>>， 所以指定类型会更好一些

    
}

fn will_sort_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    // chars.sort_unstable();
    chars.into_iter().collect()
}

fn test_will_sort_string() {
    println!(
        "eat 排序后的字符串是: {}",
        will_sort_string("eat".to_string())
    );
    println!(
        "tea 排序后的字符串是: {}",
        will_sort_string("tea".to_string())
    );
    println!(
        "tae 排序后的字符串是: {}",
        will_sort_string("tae".to_string())
    );
}

fn is_anagram_for_242(count_map: &HashMap<char, i32>, target: String) -> bool {
    // 不要用 26 个英文字符， 因为字符可能不止26个的， 改用map 来统计
    let target_count_map = statistic_char_count(target);
    for (c, count) in count_map {
        if target_count_map.get(&c).unwrap_or(&0) != count {
            return false;
        }
    }
    true

    // 如果能明确  就是 26个英文字母 ， 下面这解法 也是蛮巧妙，简单的
    //  第一个循环统计 s 中每个字符出现的次  count
    //  第二个循环  t中的字条， 出一次就减少一次,对应下标
    // 最后循环， count 数组的值 都为0 说明是异位词， 否则不是
    // char as usize， 这个转换很关键 ， 同时减  ‘a’ as usize,确定下标位置

    // let mut count = [0; 26];

    // for c in s.chars() {
    //     count[(c as usize) - ('a' as usize)] += 1;
    // }

    // for c in t.chars() {
    //     count[(c as usize) - ('a' as usize)] -= 1;
    // }

    // count.iter().all(|&x| x == 0)
}

/**
 * 这里说一下  为什么要  解引用 ， 即  *count ,   最后的 or_insert(0) 返回的是一个  &mut i32, 不能直接  加的， 所以要解引用
 * HashMap 这么设计的原因是  它own所有的值 ， 不能给值 的所有权， 不然会被拿走， 只能给一个可变引用 ， 安全地修改这个值 ， 同时保证借用规则 不被破坏。
 */
fn statistic_char_count(s: String) -> std::collections::HashMap<char, i32> {
    let mut count = std::collections::HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    count
}

fn max_i32(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    for i in 1..nums.len() {
        if nums[i] > max {
            max = nums[i];
        }
    }
    max
}

fn test_max() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let result = max_i32(nums);
    println!("最大值是: {}", result);
}

fn two_sum_01(arr: Vec<i32>, target: i32) -> Vec<usize> {
    let mut result = Vec::new();

    let mut map = std::collections::HashMap::new();

    // 有就直接返回 ， 没有就 map.inset(num,i) , 这样就保证了每个数只会被访问一次
    // map.contains_key(&complement) 这个方法的时间复杂度是 O(1)， 因为hashmap 是基于哈希表实现的， 查找一个键是否存在的平均时间复杂度是 O(1)
    for (i, &num) in arr.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            // 这个比 写 contains_key 更加高效， 因为它只需要查找一次，而 contains_key 需要查找两次， 一次是 contains_key 来检查是否存在， 一次是 get 来获取值
            ///   if map.contains_key(&complement) {
            //   result.push(map.get(&complement).unwrap().to_owned());
            result.push(index);
            result.push(i);
            return result;
        }
        map.insert(num, i);
    }
    result
}

fn test_two_sum_01() {
    let arr = vec![11, 2, 7, 15];
    let target = 9;
    let result = two_sum_01(arr, target);
    println!("[11,2, 7,  15], 目标9  结果是: {:?}", result);

    println!(
        "[2,11,15,7] 目标 17 ，结果是: {:?}",
        two_sum_01(vec![2, 11, 15, 7], 17)
    );
    println!("[3, 3] 目标6 结果是: {:?}", two_sum_01(vec![3, 3], 6));
}
