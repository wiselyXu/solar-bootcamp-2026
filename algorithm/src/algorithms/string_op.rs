// 算法题 m

// 给定一个字符串 s 和一个整数 k，从字符串开头算起，每计数至 2k 个字符，就反转这 2k 字符中的前 k 个字符。
// 如果剩余字符少于 k 个，则将剩余字符全部反转。 如果剩余字符小于 2k 但大于或等于 k 个，则反转前 k 个字符，
// 其余字符保持原样。 示例 1： 输入：s = "abcdefg", k = 2 bacdefg 输出："bacdfeg" 示例 2： 
// 输入：s = "abcd", k = 2 输出："bacd" 提示： 1 <= s.length <= 104 s 仅由小写英文组成 1 <= k <= 104
// 这个题目， 其实重点就是要将 知道标准库的api， .min    
// 字符串 转字符数组   

pub fn reverse_part(s: String,  k: i32) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let k = k as usize;  // 为什么转为usize 就便于索引呢， 索引什么。

    let mut i = 0;
    while i < n {
        // 本次结束位置
        let end = (i + 2*k).min(n);
        // 反转的位置
        let reverse_end = (i +k).min(n);

        chars[i..reverse_end].reverse();

        i = end;

    }   

    chars.into_iter().collect()

    
} 

// 反转部分  方法二， 使用步长的做法
pub fn reverse_part_2(s: String, k:i32) -> String {

    let mut chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let k = k as usize;

    for i in (0.. n).step_by(2*k) {
        let left = i;
        let right  = (i+k).min(n);
        chars[left..right].reverse();
    }

    chars.into_iter().collect()
}