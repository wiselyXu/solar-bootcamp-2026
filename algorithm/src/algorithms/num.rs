// 水仙花数： 一个数如果每个位数的同一次方和 等于 它自身。常用的是三位数， 立方， 如153 = 1³ + 5³ + 3³ = 1 + 125 + 27 = 153
// 407 = 4³ + 0³ + 7³ = 64 + 0 + 343 = 407
// 一个 n 位正整数，如果它的每一位数字的 n 次方之和正好等于它自己，这个数就叫n 位水仙花数



// num 是一个 3 位数, 其他位数的也是一样的
pub fn is_narcissistic(num: i64) -> bool {
    let n = get_len_of_num(num);
    let mut num2 = num.clone();
    let mut sum = 0;
    while num2 > 0 {
        let r = num2%10;
        num2 = num2/10;
        sum = sum + r.pow(n as u32);
    }
    num == sum
}

fn get_len_of_num(num: i64) -> i64 {
    num.abs().to_string().len() as i64
}

