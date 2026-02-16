mod algorithms;

use crate::algorithms::{sort::{
    bubble_sort, insert_sort, insertion_sort_ai, move_right, select_sort, select_sort2,
}, string_op::{reverse_part,reverse_part_2}};

fn main() {
    // let arr = vec![6, 8, 5, 7, 4];
    // let sorted_arr = bubble_sort(arr, true);
    // println!("after sort {:?}", sorted_arr);

    // let mut arr = vec![6, 8, 5, 7, 4];
    // arr = vec![6, 4, 5, 7, 4, 25, 2, 15, 7];
    // select_sort2(&mut arr);
    // println!("after sort {:?}", arr);

    // 插入排序 有点问题， 后面继续处理一下
    let mut arr = vec![6, 4, 5, 7, 4, 25, 2, 15, 7];
    insertion_sort_ai(&mut arr);

    println!("after sort {:?}", arr);
    // move_right(1, 3, &mut arr);

    //this_calc_mul();
    this_test_reverse_part();

}

fn this_calc_mul() {
    let mut a: i64 = 32679;
    let mut b: i64 = 4383;
    println!(" {} * {} = {} ", a, b, a * b);

    a = 209673;
    b = 7834;
    println!(" 209673 * 7834 = {} ", a * b);
}


fn this_test_reverse_part() {
    let s1 = "abcdefg".to_string();
    //println!("{}", reverse_part(s1, 2));
    println!("{}", reverse_part_2(s1, 2));

    let tests = vec![("abcdefg", 2, "bacdfeg"),("abcd", 2, "bacd"),
    ("1234567890", 3, "3214569870"),
    ("a", 1, "a"),
    ("abc", 4, "cba"),
    ("abcdefghijk", 3, "cbadefihgjk"),
    ("", 5, ""),
    ("abcdefgh", 100, "hgfedcba")];

    for (s, k, expected) in tests {
        //let result = reverse_part(s.to_string(), k);
        let result = reverse_part_2(s.to_string(), k);
        println!("{:8} k={} → {}  (预期: {})", s, k, result, expected);
        assert_eq!(result, expected);
    }
}
