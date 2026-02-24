mod algorithms;
mod book;

use std::io;

use crate::algorithms::{
    exam01::{get_combine_ways, get_ports, get_vans},
    list::{print_list_node, remove_nth_from_end, vec_to_list_node},
    num::is_narcissistic,
    read_helper::{read_vec, read_vec_i32_bracket},
    sort::{bubble_sort, insert_sort, insertion_sort_ai, move_right, select_sort, select_sort2},
    string_op::{reverse_part, reverse_part_2},
};

fn main() {
    test_algorithms();
    // test_book();
}

fn test_book() {
    book::fifteen::test_cons();
}

fn test_algorithms() {
    // let arr = vec![6, 8, 5, 7, 4];
    // let sorted_arr = bubble_sort(arr, true);
    // println!("after sort {:?}", sorted_arr);

    // let mut arr = vec![6, 8, 5, 7, 4];
    // arr = vec![6, 4, 5, 7, 4, 25, 2, 15, 7];
    // select_sort2(&mut arr);
    // println!("after sort {:?}", arr);

    // 插入排序 有点问题， 后面继续处理一下
    // let mut arr = vec![6, 4, 5, 7, 4, 25, 2, 15, 7];
    // insertion_sort_ai(&mut arr);

    // println!("after sort {:?}", arr);
    // move_right(1, 3, &mut arr);
    // 大数乘法
    // this_calc_mul();
    // this_test_reverse_part();

    // this_test_naccisistic();
    //test_input();
    // read_vec();
    //  read_vec_i32_bracket();
    // this_list_node_print();
    // this_exam01();
    //   this_smart_pointer_test();
    loop {
        //get_vans();
        let result = get_ports();
        println!("结果为：{:?}", result);
        // read_vec_i32_bracket(true);
    }
}

fn this_smart_pointer_test() {
    // use std::rc::Rc;
    // let a = Rc::new(5);
    // let b = Rc::clone(&a);
    // println!("a: {}, b: {}", a, b);
    let b = Box::new(5);
    let c = b;
    println!("c: {}", c);
}

fn this_exam01() {
    let arr = vec![1, 2, 3, 4, 5];
    let result = get_combine_ways(arr, 6);
    println!("{:?}", result)
}

fn this_list_node_print() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let node_head = vec_to_list_node(arr);
    let new_head = remove_nth_from_end(node_head, 4);
    print_list_node(new_head);
}

fn this_calc_mul() {
    let a: i64 = 32679;
    let b: i64 = 4383;
    println!(" {} * {} = {} ", a, b, a * b);
}

fn this_test_naccisistic() {
    let tests = vec![153, 370, 371, 407, 123, 0, 1, 1634, 8208];

    for num in tests {
        println!(
            "{} {} 水仙花数",
            num,
            if is_narcissistic(num) { "是" } else { "不" }
        )
    }
}

fn this_test_reverse_part() {
    let s1 = "abcdefg".to_string();
    //println!("{}", reverse_part(s1, 2));
    println!("{}", reverse_part_2(s1, 2));

    let tests = vec![
        ("abcdefg", 2, "bacdfeg"),
        ("abcd", 2, "bacd"),
        ("1234567890", 3, "3214569870"),
        ("a", 1, "a"),
        ("abc", 4, "cba"),
        ("abcdefghijk", 3, "cbadefihgjk"),
        ("", 5, ""),
        ("abcdefgh", 100, "hgfedcba"),
    ];

    for (s, k, expected) in tests {
        //let result = reverse_part(s.to_string(), k);
        let result = reverse_part_2(s.to_string(), k);
        println!("{:8} k={} → {}  (预期: {})", s, k, result, expected);
        assert_eq!(result, expected);
    }
}
