use openraft::entry::FromAppData;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {
    //    decimal_common_test();
    // decimal_alter_test();
    decimal_calc();
}

fn decimal_common_test() {
    let a = 1.23456789;
    let b = 2.34567890;
    let c = a + b;
    println!("{} + {} = {}", a, b, c); // 3.5802467899999995  这个明显不对， 需要使用 decimal 来处理

    println!("Using Decimal:");
    let a = Decimal::new(123456789, 8); // 1.23456789
    let b = Decimal::new(234567890, 8); // 2.34567890
    let c = a + b;
    println!("{} + {} = {}", a, b, c); // 3.58024679

    let a1 = 123456789;
    let a = Decimal::new(a1, 8); // 1.23456789
    let b = Decimal::new(234567890, 8); // 2.34567890
    let c = a + b;
    println!("decimal new 可填变量 {} + {} = {}", a, b, c); // 3.58024679

    let number = dec!(1.23456789) + dec!(2.34567890);
    println!(
        "dec!  里不能写变量 {} + {} = {}",
        dec!(-1.23),
        dec!(3.45),
        number
    ); // 2.22

    println!("common float:");
    let comm_number = a + b;
    println!("{} + {} = {}", a, b, comm_number); // 2.22
}

fn decimal_alter_test() {
    let scaled = Decimal::new(202, 2); // 1.23456789
    assert_eq!("2.02", scaled.to_string());
    assert_eq!(2.02, scaled.to_f64().unwrap());

    // from a 128 bit integer with a scale of 2
    let balance = Decimal::from_i128_with_scale(5_897_932_384_626_433_832, 2);
    assert_eq!("58979323846264338.32", balance.to_string());

    // From a string representation
    let from_string = Decimal::from_str("2.02").unwrap();
    println!("{:?}", from_string);

    assert_eq!("2.02", from_string.to_string());

    // from a string representation in a different base
    let from_string_base = Decimal::from_str_radix("ffff", 16).unwrap();
    println!("{:?}", from_string_base);
    assert_eq!("65535", from_string_base.to_string());

    let from_binary = Decimal::from_str_radix("1010", 2).unwrap();
    println!("{:?}", from_binary);
    assert_eq!("10", from_binary.to_string());

    println!("from scientific notation");
    let from_scientific = Decimal::from_scientific("1.23e4").unwrap();
    println!("{:?}", from_scientific);
    assert_eq!("12300", from_scientific.to_string());

    let from_scientific_neg = Decimal::from_scientific("1.23e-4").unwrap();
    println!("{:?}", from_scientific_neg);
    assert_eq!("0.000123", from_scientific_neg.to_string());

    // Using the raw decimal representation  ,
    // The parts are: lo, mid, hi, negative, scale , but don't need to understand the details of these parts for most use cases

    let pi = Decimal::from_parts(1_102_470_952, 185_874_565, 1_703_060_790, false, 28);
    assert_eq!("3.1415926535897932384626433832", pi.to_string());
}

fn decimal_calc() {
    let amount = dec!(25.12);
    let tax_percent = dec!(0.0085);
    let total = (amount * tax_percent) + amount;
    println!("{:?}", total);
    assert_eq!("25.33", total.round_dp(2).to_string());
}
