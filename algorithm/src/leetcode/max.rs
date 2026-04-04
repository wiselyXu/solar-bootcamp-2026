pub fn sub_main() {
    // test_max();
    test_two_sum_01();
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
    let arr = vec![11,2, 7,  15];
    let target = 9;
    let result = two_sum_01(arr, target);
    println!("[11,2, 7,  15], 目标9  结果是: {:?}", result);

    println!(
        "[2,11,15,7] 目标 17 ，结果是: {:?}",
        two_sum_01(vec![2, 11, 15, 7], 17)
    );
    println!("[3, 3] 目标6 结果是: {:?}", two_sum_01(vec![3, 3], 6));
}
