// 处理各种输入的做法， 每个算法题 都需要输入。 专门搞这一模块来处理
// 接收为字符串，数字数组， 字符串数组， 以逗号分分隔， 以空格分格等方式
use std::io::{self, BufRead};

pub fn test_input() {
    while true {
        println!("请输入任意内容：");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取失败");
        // 去掉末尾的换行符
        input = input.trim().to_string();

        println!("接收到您的输入 : {}", input);
    }
}

pub fn read_vec() {
    while true {
        println!("请输入 数组元素，  以空格分开");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取失败");
        let arr: Vec<i32> = input
            .trim()
            .split_whitespace() // 用空格分隔， 如果用 逗号等其他的分隔呢？
            .map(|s| s.parse().expect("数字解析失败"))
            .collect();

        println!("得到的数组为：{:?}", arr);
    }
}

pub fn read_vec_i32_bracket(need_prompt: bool) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();

    if need_prompt {
        println!(
            "输入数组元素， 用逗号分隔， 可以用方括号包起，也可以不用方括号， 因为不解析方括号"
        );
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    // 只留下数字， 负号， 逗号什么的
    let cleaned: String = line
        .trim()
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',' || c == '-')
        .collect();

    if cleaned.is_empty() {
        println!("输入的为空数组");
    }

    arr = cleaned
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    println!("你输入的数组为： {:?}", arr);

    arr
}

// 接收一字符串

pub fn read_line(need_prompt: bool) -> String {
    if need_prompt {
        println!("输入字符串，当作输入");
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    line.trim().to_string()
}

pub fn read_i32(need_prompt: bool) -> i32 {
    if need_prompt {
        println!("输入一数字，当作输入");
    }

    read_line(false).parse().unwrap()
}

//
pub fn read_vec_i32(need_prompt: bool) -> Vec<i32> {
    println!("请输入 数组元素，用空格分开");

    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

pub fn read_vec_string(need_prompt: bool) -> Vec<String> {
    println!("请输入 数组元素，用空格分开");

    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
