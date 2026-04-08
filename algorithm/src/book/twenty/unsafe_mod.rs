use std::os::raw::c_int;

pub fn sub_main() {
    //  println!(" from book->twenty ->unsafe_mod");
    //  raw_pointer();
    test_extern();
}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!(
        "we can use r1 and r2 in raw pointer, but need unsafe block, both point to a same memory address"
    );
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;

}

//#[link(name = "add", kind = "static")]
unsafe extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn sub(a: c_int, b: c_int) -> c_int;
}

fn test_extern() {
    let num = -3;
    unsafe {
        println!("the absolute value of {} is {}", num, abs(num));
        println!("the sum of {} and {} is {}", 5, 10, add(5, 10));

        println!("the sub of {} and {} is {}", 5, 10, sub(5, 10));
    }
}
