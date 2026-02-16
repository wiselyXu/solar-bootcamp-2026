// 方法一  冒泡排序： 反复遍历数组， 相邻元素对比
// 时间复杂度： 最差O(n^2)，  空间复杂度O(n) , 这里只是为了展示返回数组， 可以改原数组的
pub fn bubble_sort(a: Vec<u32>, ascend: bool) -> Vec<u32> {
    let mut arr = a.clone();
    let n = arr.len();
    loop {
        let mut swapped = false;
        for i in 0..(n - 1) {
            if (ascend && arr[i] > arr[i + 1]) || (!ascend && arr[i] < arr[i + 1]) {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
    arr
}

// 方法二： 选择排序： 每次都从剩余数组中选择一个最大/最小的 ， 拿当前的最小的下标位置 ， 与它交换值
//  时间复杂度  O(n^2)   , 空间复杂度O(n)
pub fn select_sort(arr: &mut Vec<u32>) {
    let len = arr.len();
    for i in 0..len {
        // 定位置  ，它与java 不一样， 是一个并闭半开区间 【0，len）

        let j = get_min_val_index(i, len, arr);
        if i != j {
            arr.swap(i, j);
        }
    }
}

pub fn select_sort2(arr: &mut Vec<u32>) {
    let len = arr.len();
    for i in 0..len {
        // 定位置  ，它与java 不一样， 是一个并闭半开区间 【0，len）
        let mut min_index = i;
        for j in i..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

fn get_min_val_index(i: usize, len: usize, arr: &Vec<u32>) -> usize {
    if i == len - 1 {
        return i;
    }

    let mut min_index = i;

    for j in (i + 1)..len {
        if arr[j] < arr[min_index] {
            min_index = j;
        }
    }

    min_index
}

// 排序算法三： 插入排序：   就像打扑克牌一样的, 将牌插入到合适的位置
// 假设前i 位是排好序的 ， 现在将第i+1 位的牌 插入到  排好序的前i 位  ， i 初始为  1
// 这个没写好， 不完全通过
pub fn insert_sort(arr: &mut Vec<usize>) {
    let len = arr.len();
    for i in 1..len {
        // 找到当前数小于等于 arr[i]  并且后面的数大于 arr[i], 将它插入，
        // 如果 arr[i] < arr[0]  直接放a[0]  后面的往后挪，
        // 如果 arr[i] >= arr[i-1]

        if (arr[i] >= arr[i - 1]) {
            continue;
        }

        if (arr[i] < arr[0]) {
            let temp = arr[i];
            move_right(0, i - 1, arr);
            arr[0] = temp;
            continue;
        }

        for j in 0..i {
            if arr[i] > arr[j] && arr[i] <= arr[j + 1] {
                let temp = arr[i];
                move_right(j, i - 1, arr);
                arr[j] = temp;
            }
        }
    }
}

pub fn move_right(start: usize, end: usize, arr: &mut Vec<usize>) {
    for i in (start..=end).rev() {
        print!("{}  ", i);
        arr[i + 1] = arr[i];
    }
}

/**
 * AI 写的比我的好， 还正确
 * 它的做法， 就是一直从后往前推， 直接找到合适的地方放为止 ， 比我从头开始看简便
 */
pub fn insertion_sort_ai(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

// // 不包含n 的
// pub fn test_loop_0n(n: i32) {
//     for i in 0..n {
//         print!("{}  ", i);
//     }

//     println!();
// }
